extern crate dotenvy;

use std::env;

use axum::Server;
use dotenvy::dotenv;
use log::info;
use tokio::signal;

// External modules reference
mod logger;
mod router;

#[tokio::main]
async fn main() {
    dotenv().ok();
    logger::init();

    // build our application with a route
    let app = router::init();

    // run it
    let app_port = env::var("APP_PORT").expect("APP_PORT env not set.");
    let addr = format!("127.0.0.1:{}", app_port);
    info!("Starting HTTP server at http://{}", addr);

    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Signal received, starting graceful shutdown");
}