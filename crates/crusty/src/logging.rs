use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup_logging() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("logs", "log.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let console_layer = fmt::layer()
        .with_target(false)
        .with_filter(LevelFilter::ERROR);

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_filter(LevelFilter::TRACE);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}
