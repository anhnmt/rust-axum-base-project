use tracing_subscriber::{
    {layer::SubscriberExt, util::SubscriberInitExt}, EnvFilter,
    fmt,
    fmt::time,
};

/// Initialize the logger
pub fn init() {
    let time_format = String::from("%Y-%m-%dT%H:%M:%S%.3f%:z");
    let time = time::ChronoLocal::with_format(time_format);

    let layer = fmt::layer()
        .with_timer(time);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(layer)
        .init();
}