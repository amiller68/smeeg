use std::sync::Arc;

use ollama_rs::generation::completion::GenerationContext;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

use smeeg::app::{Config, State};

type MyDialogue = Dialogue<MyDialogueState, InMemStorage<MyDialogueState>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum MyDialogueState {
    #[default]
    Start,
    InProgress {
        generation_context: Option<GenerationContext>,
    },
}

#[tokio::main]
async fn main() {
    let config = match Config::parse_env() {
        Ok(c) => c,
        Err(err) => {
            println!("Failed to load config: {err}");
            std::process::exit(2);
        }
    };

    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let env_filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking_writer)
        .with_filter(env_filter);

    tracing_subscriber::registry().with(stderr_layer).init();

    smeeg::register_panic_logger();
    smeeg::report_version();

    let state = match State::from_config(&config).await {
        Ok(s) => s,
        Err(err) => {
            tracing::error!("Failed to load state: {err}");
            std::process::exit(1);
        }
    };
    let state = Arc::new(state);

    let bot = Bot::new(config.telegram_bot_token());

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<MyDialogueState>, MyDialogueState>()
            .branch(dptree::case![MyDialogueState::Start].endpoint(start))
            .branch(
                dptree::case![MyDialogueState::InProgress { generation_context }]
                    .endpoint(in_progress),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<MyDialogueState>::new(), state])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn complete(
    state: Arc<State>,
    msg: Message,
    generation_context: Option<GenerationContext>,
) -> (String, Option<GenerationContext>) {
    let text = msg.text().unwrap_or_default();
    let (response, context) = state
        .agent()
        .complete(text, generation_context)
        .await
        .unwrap();
    (response, context)
}

async fn start(bot: Bot, dialogue: MyDialogue, state: Arc<State>, msg: Message) -> HandlerResult {
    let (response, generation_context) = complete(state.clone(), msg.clone(), None).await;
    bot.send_message(msg.chat.id, response).await?;
    dialogue
        .update(MyDialogueState::InProgress { generation_context })
        .await?;
    Ok(())
}

async fn in_progress(
    bot: Bot,
    dialogue: MyDialogue,
    generation_context: Option<GenerationContext>,
    state: Arc<State>,
    msg: Message,
) -> HandlerResult {
    if generation_context.is_none() {
        tracing::warn!("Generation context is missing");
    }
    println!("Context: {generation_context:?}");
    let (response, generation_context) =
        complete(state.clone(), msg.clone(), generation_context).await;
    bot.send_message(msg.chat.id, response).await?;
    dialogue
        .update(MyDialogueState::InProgress { generation_context })
        .await?;
    Ok(())
}
