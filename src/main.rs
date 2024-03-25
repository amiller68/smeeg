use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

#[tokio::main]
async fn main() {
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
    let smeeg_bot = match smeeg::SmeegBot::from_env().await {
        Ok(bot) => bot,
        Err(err) => {
            tracing::error!("Failed to create bot: {err}");
            std::process::exit(1);
        }
    };
    smeeg::run(smeeg_bot).await;
}
