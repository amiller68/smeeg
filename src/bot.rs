use std::sync::Arc;

use ollama_rs::generation::completion::GenerationContext;
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
    types::{Message, Update},
};

use crate::config::Config;
use crate::state::State;

/* Our Chat Dialogue States */

// TODO: redis?
type ChatDialogue = Dialogue<ChatDialogueState, InMemStorage<ChatDialogueState>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum ChatDialogueState {
    #[default]
    Start,
    InProgress {
        generation_context: Option<GenerationContext>,
    },
}

pub struct SmeegBot {
    bot: Bot,
    state: Arc<State>,
}

impl SmeegBot {
    pub async fn from_env() -> Result<Self, SmeegBotError> {
        let config = Config::parse_env()?;
        let (state, bot) = State::from_config(&config).await?;
        let state = Arc::new(state);
        Ok(Self { bot, state })
    }
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

async fn start(bot: Bot, dialogue: ChatDialogue, state: Arc<State>, msg: Message) -> HandlerResult {
    let (response, generation_context) = complete(state.clone(), msg.clone(), None).await;
    bot.send_message(msg.chat.id, response).await?;
    dialogue
        .update(ChatDialogueState::InProgress { generation_context })
        .await?;
    Ok(())
}

async fn in_progress(
    bot: Bot,
    dialogue: ChatDialogue,
    generation_context: Option<GenerationContext>,
    state: Arc<State>,
    msg: Message,
) -> HandlerResult {
    if generation_context.is_none() {
        tracing::warn!("Generation context is missing");
    }
    let (response, generation_context) =
        complete(state.clone(), msg.clone(), generation_context).await;
    bot.send_message(msg.chat.id, response).await?;
    dialogue
        .update(ChatDialogueState::InProgress { generation_context })
        .await?;
    Ok(())
}

pub async fn run(bot: SmeegBot) {
    Dispatcher::builder(
        bot.bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<ChatDialogueState>, ChatDialogueState>()
            .branch(dptree::case![ChatDialogueState::Start].endpoint(start))
            .branch(
                dptree::case![ChatDialogueState::InProgress { generation_context }]
                    .endpoint(in_progress),
            ),
    )
    .dependencies(dptree::deps![
        InMemStorage::<ChatDialogueState>::new(),
        bot.state
    ])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

#[derive(Debug, thiserror::Error)]
pub enum SmeegBotError {
    #[error("agent error: {0}")]
    Agent(#[from] crate::agent::AgentError),
    #[error("config error: {0}")]
    Config(#[from] crate::config::ConfigError),
    #[error("state setup error: {0}")]
    StateSetup(#[from] crate::state::StateSetupError),
}
