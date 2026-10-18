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

//{
//  "data": {
//    "total_seconds": 3600,
//    "daily_average": 1800,
//    "languages": [
//      {
//        "name": "Ruby",
//        "total_seconds": 2400,
//        "percent": 66.6
//      }
//    ],
//    "projects": [
//      {
//        "name": "hackatime",
//        "total_seconds": 3600,
//        "percent": 100
//      }
//    ],
//    "editors": [
//      {
//        "name": "VS Code",
//        "total_seconds": 3600,
//        "percent": 100
//      }
//    ],
//    "streak": 7,
//    "unique_total_seconds": 12960
//  },
//  "trust_factor": {
//    "trust_level": "blue",
//    "trust_value": 0
//  }
//}
//
#[derive(Debug, Deserialize)]
pub struct TrustFactor {
    pub trust_level: String,
    pub trust_value: usize,
}

#[derive(Debug, Deserialize)]
pub struct Stats {
    pub name: String,
    pub total_seconds: u64,
    pub percent: f64,
}
#[derive(Debug, Deserialize)]
pub struct Data {
    pub total_seconds: u64,
    pub daily_average: u64,
    pub languages: Vec<Stats>,
    pub projects: Vec<Stats>,
    pub editors: Vec<Stats>,
    pub streak: u64,
    pub unique_total_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct StatsResponse {
    pub data: Data,
    pub trust_factor: TrustFactor,
}
