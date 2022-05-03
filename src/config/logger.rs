use tracing_subscriber::{
    {layer::SubscriberExt, util::SubscriberInitExt}, EnvFilter,
    fmt::{self, time::LocalTime},
};

/// Initialize the logger
pub fn init() {
    let timer = LocalTime::rfc_3339();

    let layer = fmt::layer()
        // .json()
        .with_timer(timer);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(layer)
        .init();
}