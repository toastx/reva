use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::structs::*;

const API_BASE: &str = "https://api-dinodial-proxy.cyces.co";

pub async fn make_call(
    State(client): State<reqwest::Client>,
    Json(body): Json<MakeCallRequest>,
) -> Result<Json<MakeCallResponse>, StatusCode> {
    let token = std::env::var("ADMIN_TOKEN").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
        Err(status)
    }
}

pub async fn list_calls(
    State(client): State<reqwest::Client>,
) -> Result<Json<ListCallsResponse>, StatusCode> {
    let token = std::env::var("ADMIN_TOKEN").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let res = client
        .get(format!("{}/api/proxy/calls/list/", API_BASE))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let json = res.json::<ListCallsResponse>().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    Ok(Json(json))
}

pub async fn call_detail(
    State(client): State<reqwest::Client>,
    Path(call_id): Path<String>,
) -> Result<Json<CallDetailResponse>, StatusCode> {
    let token = std::env::var("ADMIN_TOKEN").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let res = client
        .get(format!("{}/api/proxy/call/detail/{}/", API_BASE, call_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let json = res.json::<CallDetailResponse>().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    Ok(Json(json))
}

pub async fn recording_url(
    State(client): State<reqwest::Client>,
    Path(call_id): Path<String>,
) -> Result<Json<RecordingUrlResponse>, StatusCode> {
    let token = std::env::var("ADMIN_TOKEN").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let res = client
        .get(format!("{}/api/proxy/recording-url/{}/", API_BASE, call_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let json = res.json::<RecordingUrlResponse>().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    Ok(Json(json))
}
