use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::structs::*;
use std::fs;

const API_BASE: &str = "https://api-dinodial-proxy.cyces.co";

pub async fn make_call(
    State(client): State<reqwest::Client>,
) -> Result<Json<MakeCallResponse>, StatusCode> {
    println!("📞 make_call endpoint triggered");
    
    let token = std::env::var("ADMIN_TOKEN").map_err(|e| {
        eprintln!("❌ Error: Failed to get ADMIN_TOKEN - {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let prompt = fs::read_to_string("src/prompts/prompt.txt").map_err(|e| {
        eprintln!("❌ Error: Failed to read prompt.txt - {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let evaluation_tool_str = fs::read_to_string("src/evaluation_tools/evaluation_tool.json").map_err(|e| {
        eprintln!("❌ Error: Failed to read evaluation_tool.json - {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let evaluation_tool: serde_json::Value = serde_json::from_str(&evaluation_tool_str).map_err(|e| {
        eprintln!("❌ Error: Failed to parse evaluation_tool JSON - {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let body = MakeCallRequest {
        prompt,
        evaluation_tool,
        vad_engine: Some("CALGAR".to_string()),
    };
    
    let res = client
        .post(format!("{}/api/proxy/make-call/", API_BASE))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            eprintln!("❌ Error: Failed to send request to backend - {}", e);
            StatusCode::BAD_GATEWAY
        })?;
    
    let status = res.status();
    
    let response_text = res.text().await.map_err(|e| {
        eprintln!("❌ Error: Failed to get response text - {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    println!("📊 Raw Response:\n{}", response_text);
    
    let json: MakeCallResponse = serde_json::from_str(&response_text).map_err(|e| {
        eprintln!("❌ Error: Failed to parse response - {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    if status.is_success() {
        Ok(Json(json))
    } else {
        eprintln!("❌ Error: Backend returned non-success status - {}", status);
        Err(StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
    }
}

pub async fn list_calls(
    State(client): State<reqwest::Client>,
) -> Result<Json<ListCallsResponse>, StatusCode> {
    println!("📋 list_calls endpoint triggered");
    
    let token = std::env::var("ADMIN_TOKEN").map_err(|e| {
        eprintln!("❌ Error: Failed to get ADMIN_TOKEN - {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let res = client
        .get(format!("{}/api/proxy/calls/list/", API_BASE))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| {
            eprintln!("❌ Error: Failed to fetch calls list - {}", e);
            StatusCode::BAD_GATEWAY
        })?;
    
    let response_text = res.text().await.map_err(|e| {
        eprintln!("❌ Error: Failed to get response text - {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    println!("📊 Raw Response:\n{}", response_text);
    
    let json: ListCallsResponse = serde_json::from_str(&response_text).map_err(|e| {
        eprintln!("❌ Error: Failed to parse calls list - {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    Ok(Json(json))
}

pub async fn call_detail(
    State(client): State<reqwest::Client>,
    Path(call_id): Path<String>,
) -> Result<Json<CallDetailResponse>, StatusCode> {
    println!("🔍 call_detail endpoint triggered for call_id: {}", call_id);
    
    let token = std::env::var("ADMIN_TOKEN").map_err(|e| {
        eprintln!("❌ Error: Failed to get ADMIN_TOKEN - {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let res = client
        .get(format!("{}/api/proxy/call/detail/{}/", API_BASE, call_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| {
            eprintln!("❌ Error: Failed to fetch call detail - {}", e);
            StatusCode::BAD_GATEWAY
        })?;
    
    let response_text = res.text().await.map_err(|e| {
        eprintln!("❌ Error: Failed to get response text - {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    println!("📊 Raw Response:\n{}", response_text);
    
    let json: CallDetailResponse = serde_json::from_str(&response_text).map_err(|e| {
        eprintln!("❌ Error: Failed to parse call detail - {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    Ok(Json(json))
}

pub async fn recording_url(
    State(client): State<reqwest::Client>,
    Path(call_id): Path<String>,
) -> Result<Json<RecordingUrlResponse>, StatusCode> {
    println!("🎵 recording_url endpoint triggered for call_id: {}", call_id);
    
    let token = std::env::var("ADMIN_TOKEN").map_err(|e| {
        eprintln!("❌ Error: Failed to get ADMIN_TOKEN - {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let res = client
        .get(format!("{}/api/proxy/recording-url/{}/", API_BASE, call_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| {
            eprintln!("❌ Error: Failed to fetch recording URL - {}", e);
            StatusCode::BAD_GATEWAY
        })?;
    
    let response_text = res.text().await.map_err(|e| {
        eprintln!("❌ Error: Failed to get response text - {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    println!("📊 Raw Response:\n{}", response_text);
    
    let json: RecordingUrlResponse = serde_json::from_str(&response_text).map_err(|e| {
        eprintln!("❌ Error: Failed to parse recording URL - {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    Ok(Json(json))
}
