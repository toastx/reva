use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============ MAKE CALL STRUCTS ============

#[derive(Deserialize, Serialize, Debug)]
pub struct MakeCallRequest {
    pub prompt: String,
    pub evaluation_tool: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad_engine: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MakeCallResponse {
    pub data: CallData,
    pub status: String,
    pub status_code: u16,
    pub action_code: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CallData {
    pub id: u64,
    pub message: String,
}

// ============ LIST CALLS STRUCTS ============

#[derive(Deserialize, Serialize, Debug)]
pub struct ListCallsResponse {
    pub data: ListCallsData,
    pub status: String,
    pub status_code: u16,
    pub action_code: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListCallsData {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<CallItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CallItem {
    pub id: u64,
    pub call_id: String,
    pub created: String,
    pub phone_number: String,
    pub status: String,
}

// ============ CALL DETAIL STRUCTS ============

#[derive(Deserialize, Serialize, Debug)]
pub struct CallDetailResponse {
    pub data: CallDetailData,
    pub status: String,
    pub status_code: u16,
    pub action_code: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CallDetailData {
    pub id: u64,
    pub call_id: String,
    pub created: String,
    pub phone_number: String,
    pub status: String,
    pub prompt: String,
    pub evaluation_tool: Value,
    pub exotel_id: String,
    pub call_details: CallDetails,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CallDetails {
    #[serde(rename = "callId")]
    pub call_id: String,
    pub events: Vec<Event>,
    #[serde(rename = "llmNotes")]
    pub llm_notes: Vec<Value>,
    #[serde(rename = "toolCalls")]
    pub tool_calls: Vec<ToolCall>,
    #[serde(rename = "phaseHistory")]
    pub phase_history: Vec<PhaseHistory>,
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: Option<UsageMetadata>,  // Changed to Option
    #[serde(rename = "callSummaryData")]
    pub call_summary_data: Option<Value>,  // Changed to Option
    #[serde(rename = "callOutcomesData")]
    pub call_outcomes_data: Option<Value>,  // Changed to Option
    #[serde(rename = "terminationReason")]
    pub termination_reason: Option<String>,  // Changed to Option
    #[serde(rename = "terminationSource")]
    pub termination_source: Option<String>,  // Changed to Option
    #[serde(rename = "transcriptionData")]
    pub transcription_data: TranscriptionData,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub data: Value,
    pub event: String,
    pub timestamp: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ToolCall {
    #[serde(rename = "toolName")]
    pub tool_name: String,
    pub timestamp: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PhaseHistory {
    pub to: String,
    pub from: String,
    pub metadata: Value,
    pub timestamp: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UsageMetadata {
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: u32,
    #[serde(rename = "promptTokenCount")]
    pub prompt_token_count: u32,
    #[serde(rename = "promptTokensDetails")]
    pub prompt_tokens_details: Vec<TokenDetail>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenDetail {
    pub modality: String,
    #[serde(rename = "tokenCount")]
    pub token_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TranscriptionData {
    pub transcripts: Vec<Value>,
    #[serde(rename = "interruptionTimestamps")]
    pub interruption_timestamps: Vec<i64>,
    #[serde(rename = "turnCompleteTimestamps")]
    pub turn_complete_timestamps: Vec<i64>,
    #[serde(rename = "generationCompleteTimestamps")]
    pub generation_complete_timestamps: Vec<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RecordingUrlResponse {
    pub data: RecordingUrlData,
    pub status: String,
    pub status_code: u16,
    pub action_code: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RecordingUrlData {
    pub recording_url: Option<String>,  // Changed to Option
}