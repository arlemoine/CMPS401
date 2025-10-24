// NOTE: This is currently only a skeleton which opens a websocket connection to the frontend and echoes messages received from the frontend back to the frontend.
mod ws;
mod config;

use axum::{
    routing::get, 
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber;
use tokio::net::TcpListener;

use config::Config;


#[tokio::main]
async fn main() {
    // Pull from config file
    let config = Config::default();

    // Init logger
    tracing_subscriber::fmt()
        .with_max_level(config.log_level)
        .init();

    // Define socket address and utilize logger
    let addr: SocketAddr = config.socket_addr();
    tracing::info!("listening on {}", addr);

    // Define router paths
    let app = Router::new().route("/ws", get(ws::ws_handler));

    // Define server connection
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
