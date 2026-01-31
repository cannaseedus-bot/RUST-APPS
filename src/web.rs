use crate::ai::AIModel;
use crate::project::Project;
use anyhow::{Context, Result};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use warp::ws::{WebSocket, Ws};
use warp::{Filter, Rejection, Reply};

#[derive(Clone)]
pub struct WebState {
    pub projects: Arc<RwLock<Vec<Project>>>,
    pub ai_model: Arc<RwLock<Option<AIModel>>>,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateRequest {
    pub prompt: String,
    pub framework: String,
    pub model: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateResponse {
    pub code: String,
    pub tokens: usize,
    pub time_ms: u64,
}

pub async fn start_web_server(port: u16, host: &str, enable_ai: bool) -> Result<()> {
    let state = WebState {
        projects: Arc::new(RwLock::new(Vec::new())),
        ai_model: Arc::new(RwLock::new(None)),
    };

    if enable_ai {
        println!("🤖 Loading AI model for web interface...");
        let mut ai_model = AIModel::new("phi-3-mini").await?;
        ai_model.load().await?;
        *state.ai_model.write().await = Some(ai_model);
        println!("✅ AI model loaded for web interface");
    }

    let static_files = warp::path("static").and(warp::fs::dir("./static"));

    let api = warp::path("api");

    let generate = api
        .and(warp::path("generate"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(handle_generate);

    let projects = api
        .and(warp::path("projects"))
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(list_projects);

    let ws = warp::path("ws")
        .and(warp::ws())
        .and(with_state(state.clone()))
        .map(|ws: Ws, state: WebState| ws.on_upgrade(move |socket| handle_websocket(socket, state)));

    let index = warp::path::end().map(|| warp::reply::html(include_str!("../templates/index.html")));

    let routes = index
        .or(static_files)
        .or(generate)
        .or(projects)
        .or(ws)
        .with(warp::cors().allow_any_origin())
        .with(warp::log("nexus_web"));

    let addr: std::net::SocketAddr = format!("{}:{}", host, port)
        .parse()
        .context("Invalid host/port")?;

    println!("🌐 Nexus Studio Web Interface");
    println!("   URL: http://{}:{}", host, port);
    println!("   AI Enabled: {}", enable_ai);
    println!("\n🚀 Server starting...");

    warp::serve(routes).run(addr).await;

    Ok(())
}

fn with_state(
    state: WebState,
) -> impl Filter<Extract = (WebState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

async fn handle_generate(request: GenerateRequest, state: WebState) -> Result<impl Reply, Rejection> {
    let response = match &mut *state.ai_model.write().await {
        Some(ai_model) => {
            let ai_response = ai_model
                .generate(&request.prompt, 2000)
                .await
                .map_err(|e| warp::reject::custom(ApiError::from(e)))?;

            GenerateResponse {
                code: ai_response.content,
                tokens: ai_response.tokens,
                time_ms: ai_response.time_ms,
            }
        }
        None => GenerateResponse {
            code: format!(
                "// AI not enabled\n// Request: {}\n// Framework: {}",
                request.prompt, request.framework
            ),
            tokens: 0,
            time_ms: 0,
        },
    };

    Ok(warp::reply::json(&response))
}

async fn list_projects(state: WebState) -> Result<impl Reply, Rejection> {
    let projects = state.projects.read().await;
    Ok(warp::reply::json(&*projects))
}

async fn handle_websocket(ws: WebSocket, state: WebState) {
    let (mut tx, mut rx) = ws.split();
    let client_id = Uuid::new_v4();

    println!("📡 WebSocket connected: {}", client_id);

    let _ = tx
        .send(warp::ws::Message::text(
            serde_json::json!({
                "type": "connected",
                "client_id": client_id.to_string(),
                "message": "Connected to Nexus Studio AI"
            })
            .to_string(),
        ))
        .await;

    while let Some(result) = rx.next().await {
        match result {
            Ok(msg) => {
                if let Ok(text) = msg.to_str() {
                    handle_websocket_message(text, &mut tx, &state).await;
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }

    println!("📡 WebSocket disconnected: {}", client_id);
}

async fn handle_websocket_message(
    text: &str,
    tx: &mut futures::stream::SplitSink<WebSocket, warp::ws::Message>,
    state: &WebState,
) {
    match serde_json::from_str::<serde_json::Value>(text) {
        Ok(data) => {
            if let Some(msg_type) = data.get("type").and_then(|t| t.as_str()) {
                match msg_type {
                    "generate" => {
                        if let (Some(prompt), Some(framework)) = (
                            data.get("prompt").and_then(|p| p.as_str()),
                            data.get("framework").and_then(|f| f.as_str()),
                        ) {
                            handle_ws_generate(prompt, framework, tx, state).await;
                        }
                    }
                    _ => {
                        let _ = tx
                            .send(warp::ws::Message::text(
                                serde_json::json!({
                                    "type": "error",
                                    "message": format!("Unknown message type: {}", msg_type)
                                })
                                .to_string(),
                            ))
                            .await;
                    }
                }
            }
        }
        Err(e) => {
            let _ = tx
                .send(warp::ws::Message::text(
                    serde_json::json!({
                        "type": "error",
                        "message": format!("Invalid JSON: {}", e)
                    })
                    .to_string(),
                ))
                .await;
        }
    }
}

async fn handle_ws_generate(
    prompt: &str,
    framework: &str,
    tx: &mut futures::stream::SplitSink<WebSocket, warp::ws::Message>,
    state: &WebState,
) {
    let _ = tx
        .send(warp::ws::Message::text(
            serde_json::json!({
                "type": "generating",
                "message": "AI is generating code..."
            })
            .to_string(),
        ))
        .await;

    let response = match &mut *state.ai_model.write().await {
        Some(ai_model) => match ai_model.generate(prompt, 2000).await {
            Ok(ai_response) => serde_json::json!({
                "type": "generated",
                "code": ai_response.content,
                "tokens": ai_response.tokens,
                "time_ms": ai_response.time_ms,
                "model": ai_response.model
            }),
            Err(e) => serde_json::json!({
                "type": "error",
                "message": format!("AI generation failed: {}", e)
            }),
        },
        None => serde_json::json!({
            "type": "error",
            "message": "AI model not available"
        }),
    };

    let _ = tx.send(warp::ws::Message::text(response.to_string())).await;
}

#[derive(Debug)]
struct ApiError(anyhow::Error);

impl warp::reject::Reject for ApiError {}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError(err)
    }
}
