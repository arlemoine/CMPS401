// NOTE: This is currently only a skeleton which opens a websocket connection to the frontend and echoes messages received from the frontend back to the frontend.

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse, 
    routing::get, 
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber;
use tokio::net::TcpListener;

/// Receives initial GET from client (directed to ws), drops the http protocol, and establishes websocket connection
async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

/// Defines websocket communication after ws_handler() is called
async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(t) => {
                // echo
                if socket.send(Message::Text(t)).await.is_err() {
                    break;
                }
            }
            Message::Binary(b) => {
                if socket.send(Message::Binary(b)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    // Init logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Define router paths and socket address
    let app = Router::new().route("/ws", get(ws_handler));
    let addr = SocketAddr::from(([127,0,0,1], 3000));

    // Utilize logger
    tracing::info!("listening on {}", addr);

    // Define server connection
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
