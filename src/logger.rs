use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::fmt::time;

/// Initialize the logger
pub fn init() {
    // App mode
    tracing_subscriber::registry()
        .with(fmt::layer().with_timer(local_time()))
        .with(EnvFilter::from_default_env())
        .init();
}

/// Local time
fn local_time() -> time::ChronoLocal {
    time::ChronoLocal::with_format(time_format())
}

/// Time format
fn time_format() -> String {
    String::from("%Y-%m-%dT%H:%M:%S%.3f%:z")
}