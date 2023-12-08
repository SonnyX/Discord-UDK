mod udk_log;
mod discord;
mod dll;
mod error;

pub fn one_time_initialization() {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::WARN)
        .init();
}