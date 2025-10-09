use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;
use serde::{Deserialize, Serialize};
use rand::Rng;

/// ---- JSON message types (first step) ----
/// `type` and `payload` fields.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
enum ClientMsg {
    /// { "type": "join", "payload": { "displayName": "Adam" } }
    #[serde(rename = "join")]
    Join { #[serde(rename = "displayName")] display_name: String },

    /// { "type": "create_match", "payload": {} }
    #[serde(rename = "create_match")]
    CreateMatch {},
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
enum ServerMsg {
    /// { "type": "hello", "payload": { "serverVersion": "0.1" } }
    #[serde(rename = "hello")]
    Hello { #[serde(rename = "serverVersion")] server_version: &'static str },

    /// { "type": "match_created", "payload": { "matchId": "ABCD" } }
    #[serde(rename = "match_created")]
    MatchCreated { #[serde(rename = "matchId")] match_id: String },

    /// { "type": "error", "payload": { "code": "BAD_JSON", "message": "..." } }
    #[serde(rename = "error")]
    Error { code: String, message: String },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/ws", get(ws_upgrade));
    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    info!(addr = %listener.local_addr().unwrap(), "server listening");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "OK"
}

async fn ws_upgrade(ws: WebSocketUpgrade) -> impl IntoResponse {
    info!("/ws upgrade requested");
    ws.on_upgrade(handle_socket)
}

fn new_match_id() -> String {
    let mut rng = rand::rng();
    (0..4).map(|_| rng.random_range(b'A'..=b'Z') as char).collect()
}

async fn handle_socket(mut socket: WebSocket) {
    info!("websocket connected");
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(txt) = msg {
            info!(%txt, "received text");
            match serde_json::from_str::<ClientMsg>(&txt) {
                Ok(ClientMsg::Join { display_name }) => {
                    info!(%display_name, "join message parsed");
                    let out = ServerMsg::Hello { server_version: "0.1" };
                    let s = serde_json::to_string(&out).unwrap();
                    if socket.send(Message::Text(s.into())).await.is_err() { break; }
                }
                Ok(ClientMsg::CreateMatch {}) => {
                    let id = new_match_id();
                    info!(%id, "create_match parsed; issuing match_id");
                    let out = ServerMsg::MatchCreated { match_id: id };
                    let s = serde_json::to_string(&out).unwrap();
                    if socket.send(Message::Text(s.into())).await.is_err() { break; }
                }
                Err(e) => {
                    warn!(error = %e, "bad json");
                    let out = ServerMsg::Error { code: "BAD_JSON".into(), message: e.to_string() };
                    let s = serde_json::to_string(&out).unwrap();
                    if socket.send(Message::Text(s.into())).await.is_err() { break; }
                }
            }
        }
    }
    info!("websocket closed");
}
