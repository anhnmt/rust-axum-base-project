extern crate dotenvy;

use std::env;

use axum::Server;
use dotenvy::dotenv;
use log::info;
use tokio::signal;

use crate::config::{
    database::Database,
    logger,
    router,
};

// External modules reference
mod config;
mod controllers;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();
    logger::init();

    let db = match Database::new().await {
        Ok(value) => value,
        Err(_) => panic!("Failed to setup database connection"),
    };

    // Print the databases in our MongoDB cluster:
    tokio::spawn(async move {
        info!("Databases:");
        for name in db.client.list_database_names(None, None).await.unwrap() {
            info!("- {}", name);
        }
    });

    // build our application with a route
    let app = router::init();

    // run it
    let app_port = env::var("APP_PORT").expect("APP_PORT env not set.");
    info!("Starting HTTP server at http://localhost:{}", app_port);

    let addr = format!("0.0.0.0:{}", app_port);
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