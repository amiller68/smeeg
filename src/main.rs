use teloxide::prelude::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

use smeeg::app::{Config, State};

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

    let _state = match State::from_config(&config).await {
        Ok(s) => s,
        Err(err) => {
            tracing::error!("Failed to load state: {err}");
            std::process::exit(1);
        }
    };

    let bot = Bot::new(config.telegram_bot_token().clone());

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
