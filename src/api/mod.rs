// This module is for all the api related stuff

//Let's get started with fetching some basic info first.
//
pub mod analytics;
use std::{collections::HashMap, error::Error};

use reqwest::StatusCode;

use crate::api::models::{Heartbeat, HeartbeatResponse, StatsResponse};

pub mod models;
const BASE_URL: &str = "https://hackatime.hackclub.com";

pub fn parse_heartbeats(json_str: &str) -> Result<HeartbeatResponse, serde_json::Error> {
    serde_json::from_str(json_str)
}

pub async fn fetch_heartbeats(
    key: &str,
    start: &str,
    end: Option<&str>,
) -> Result<HeartbeatResponse, Box<dyn Error>> {
    let url = format!("{BASE_URL}/api/v1/my/heartbeats");

    let mut query = vec![("start_time", start)];
    if let Some(end) = end {
        query.push(("end_time", end));
    }
    //curl 'https://hackatime.hackclub.com/api/v1/my/heartbeats?start_time=&end_time=' \
    // --header 'Authorization: Bearer YOUR_SECRET_TOKEN'
    let resp = reqwest::Client::new()
        .get(&url)
        .bearer_auth(key)
        .query(&query)
        .send()
        .await?;

    let status = resp.status();
    if status == StatusCode::UNAUTHORIZED {
        return Err("401 Unauthorized: Invalid API Key".into());
    }

    let text = resp.text().await?;
    let data = parse_heartbeats(&text)?;
    Ok(data)
}

pub async fn fetch_stats(
    key: &str,
    start: Option<&str>,
    end: Option<&str>,
) -> Result<StatsResponse, Box<dyn std::error::Error>> {
    let url = format!("{BASE_URL}/api/v1/users/my/stats");

    let mut query = vec![("features", "languages,projects,editors")];
    if let Some(start) = start {
        query.push(("start_date", start));
    }
    if let Some(end) = end {
        query.push(("end_date", end));
    }

    let resp = reqwest::Client::new()
        .get(&url)
        .bearer_auth(key)
        .query(&query)
        .send()
        .await?;

    Ok(resp.json::<StatsResponse>().await?)
}

pub fn count_os(hbs: &[Heartbeat]) -> (Vec<(String, u64)>, u64) {
    let mut counts: HashMap<String, u64> = HashMap::new();
    let mut missing = 0;
    for hb in hbs {
        match &hb.operating_system {
            Some(raw) => {
                let name = raw.to_lowercase();
                *counts.entry(name).or_insert(0) += 1;
            }
            None => missing += 1,
        }
    }

    let mut sorted: Vec<(String, u64)> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    (sorted, missing)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_valid_payload() {
        let sample = r#"{
            "start_time": "2024-03-20T00:00:00Z",
            "end_time": "2024-03-20T23:59:59Z",
            "total_seconds": 3600,
            "heartbeats": [
                {
                    "id": 1024,
                    "entity": "/Users/orpheus/test.rs",
                    "type": "file",
                    "category": "coding",
                    "time": 1709251200,
                    "project": "hackatime",
                    "branch": "main",
                    "language": "Rust",
                    "is_write": true
                }
            ]
        }"#;

        let result = parse_heartbeats(sample);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

        let data = result.unwrap();
        assert_eq!(data.total_seconds, 3600);
        assert_eq!(data.heartbeats.len(), 1);
        assert_eq!(data.heartbeats[0].project.as_deref(), Some("hackatime"));
    }

    #[test]
    fn parse_null_values() {
        let sample = r#"{
            "start_time": "2024-03-20T00:00:00Z",
            "end_time": "2024-03-20T23:59:59Z",
            "total_seconds": 0,
            "heartbeats": [
                {
                    "id": 1024,
                    "entity": null,
                    "time": 1709251200
                }
            ]
        }
        "#;

        let result = parse_heartbeats(sample);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

        let data = result.unwrap();
        assert_eq!(data.heartbeats[0].entity, None);
        assert_eq!(data.heartbeats[0].project, None);
    }

    #[tokio::test]
    #[ignore]
    async fn live_fetch_works() {
        let key = crate::config::load_key().expect("Failed to load the key for live test");
        let res =
            fetch_heartbeats(&key, "2026-10-05T00:00:00Z", Some("2026-10-11T23:59:59Z")).await;
        assert!(res.is_ok(), "Live fetch error: {:?}", res.err());
    }
}
