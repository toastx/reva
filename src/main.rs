use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;

const API_BASE: &str = "https://api-dinodial-proxy.cyces.co";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/proxy/make-call/", post(make_call))
        .route("/api/proxy/calls/list/", get(list_calls))
        .route("/api/proxy/call/detail/:call_id/", get(call_detail));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ============ MAKE CALL STRUCTS ============

#[derive(Deserialize, Serialize)]
struct MakeCallRequest {
    prompt: String,
    evaluation_tool: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    vad_engine: Option<Value>,
}

#[derive(Deserialize, Serialize)]
struct MakeCallResponse {
    data: CallData,
    status: String,
    status_code: u16,
    action_code: String,
}

#[derive(Deserialize, Serialize)]
struct CallData {
    id: u64,
    message: String,
}

// ============ LIST CALLS STRUCTS ============

#[derive(Deserialize, Serialize)]
struct ListCallsResponse {
    data: ListCallsData,
    status: String,
    status_code: u16,
    action_code: String,
}

#[derive(Deserialize, Serialize)]
struct ListCallsData {
    count: u32,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<CallItem>,
}

#[derive(Deserialize, Serialize)]
struct CallItem {
    id: u64,
    call_id: String,
    created: String,
    phone_number: String,
    status: String,
}

// ============ ENDPOINTS ============

async fn make_call(
    headers: HeaderMap,
    Json(body): Json<MakeCallRequest>,
) -> Result<Json<MakeCallResponse>, StatusCode> {
    let token = extract_bearer_token(&headers)?;
    
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/proxy/make-call/", API_BASE))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .json(&body)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let status = res.status();
    let json = res.json::<MakeCallResponse>().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    if status.is_success() {
        Ok(Json(json))
    } else {
        Err(StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
    }
}

async fn list_calls(headers: HeaderMap) -> Result<Json<ListCallsResponse>, StatusCode> {
    let token = extract_bearer_token(&headers)?;
    
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/api/proxy/calls/list/", API_BASE))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let json = res.json::<ListCallsResponse>().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    Ok(Json(json))
}

async fn call_detail(
    headers: HeaderMap,
    Path(call_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let token = extract_bearer_token(&headers)?;
    
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/api/proxy/call/detail/{}/", API_BASE, call_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let json = res.json::<Value>().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    Ok(Json(json))
}

fn extract_bearer_token(headers: &HeaderMap) -> Result<String, StatusCode> {
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    if let Some(token) = auth_header.strip_prefix("Bearer ") {
        Ok(token.to_string())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
