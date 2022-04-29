use std::env;

use tracing_subscriber::fmt::time;
use tracing_subscriber::util::SubscriberInitExt;

/// Initialize the logger
pub fn init() {
    // App mode
    match env::var("APP_DEV") {
        Ok(s) => {
            if s.eq_ignore_ascii_case("true") {
                return dev_logger();
            }

            return prod_logger();
        }
        _ => prod_logger(),
    }
}

/// Development mode
fn dev_logger() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_timer(local_time())
        .finish()
        .init();
}

/// Production mode
fn prod_logger() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_timer(utc_time())
        .json()
        .finish()
        .init();
}

/// UTC time
fn utc_time() -> time::ChronoUtc {
    time::ChronoUtc::with_format(time_format())
}

/// Local time
fn local_time() -> time::ChronoLocal {
    time::ChronoLocal::with_format(time_format())
}

/// Time format
fn time_format() -> String {
    String::from("%Y-%m-%dT%H:%M:%S%.3f%:z")
}