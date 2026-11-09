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

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct StatItem {
    pub name: String,
    pub total_seconds: f64,
}

#[derive(Debug, Deserialize)]
pub struct Stats {
    pub user_id: String,
    pub total_seconds: f64,
    pub projects: Vec<StatItem>,
    pub languages: Vec<StatItem>,
    pub editors: Vec<StatItem>,
    pub operating_systems: Vec<StatItem>,
    pub categories: Vec<StatItem>,
}

#[derive(Debug, Deserialize)]
pub struct Today {
    pub grand_total: Total,
    pub goal: Option<Goal>,
}

#[derive(Debug, Deserialize)]
pub struct Total {
    pub total_seconds: f64,
}

#[derive(Debug, Deserialize)]
pub struct Goal {
    pub target_seconds: f64,
}

#[derive(Debug, Deserialize)]
pub struct Daily {
    pub grand_total: Total,
    pub projects: Vec<StatItem>,
    pub range: DayRange,
}

#[derive(Debug, Deserialize)]
pub struct DayRange {
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct SpanResponse {
    pub spans: Vec<ActivitySpan>,
}

#[derive(Debug, Deserialize)]
pub struct ActivitySpan {
    pub start_time: f64,
    pub end_time: f64,
}

#[derive(Debug, Deserialize)]
pub struct ProjectResponse {
    pub projects: Vec<ProjectInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub total_seconds: f64,
    pub languages: Vec<String>,
}
