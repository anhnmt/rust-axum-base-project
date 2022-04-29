extern crate dotenvy;

use std::net::SocketAddr;

use axum::{response::Html, Router, routing::get};
use axum::http::{Method};
use dotenvy::dotenv;
use log::info;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// External modules reference
mod logger;

#[tokio::main]
async fn main() {
    dotenv().ok();
    logger::init();

    // build our application with a route
    let app = Router::new().route("/", get(handler)).layer(
        // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
        // for more details
        //
        // pay attention that for some request types like posting content-type: application/json
        // it is required to add ".allow_headers(vec![http::header::CONTENT_TYPE])"
        // or see this issue https://github.com/tokio-rs/axum/issues/849
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS]),
    );

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
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