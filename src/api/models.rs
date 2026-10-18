use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HeartbeatResponse {
    pub start_time: String,
    pub end_time: String,
    pub total_seconds: u64,
    pub heartbeats: Vec<Heartbeat>,
}

#[derive(Debug, Deserialize)]
pub struct Heartbeat {
    pub id: u64,
    pub entity: Option<String>,
    #[serde(rename = "type")]
    pub entity_type: Option<String>,
    pub category: Option<String>,
    pub time: f64,
    pub project: Option<String>,
    pub branch: Option<String>,
    pub language: Option<String>,
    pub is_write: Option<bool>,
    pub editor: Option<String>,
    pub operating_system: Option<String>,
    pub machine: Option<String>,
    pub cursorpos: Option<u64>,
    pub lineno: Option<u64>,
    pub lines: Option<u64>,
    pub line_additions: Option<u64>,
    pub line_deletions: Option<u64>,
}
