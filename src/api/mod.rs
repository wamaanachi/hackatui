// This module is for all the api related stuff

//Let's get started with fetching some basic info first.
//

use std::error::Error;

use reqwest::StatusCode;

use crate::api::models::HeartbeatResponse;

pub mod models;
const BASE_URL: &str = "https://hackatime.hackclub.com";

pub fn parse_heartbeats(json_str: &str) -> Result<HeartbeatResponse, serde_json::Error> {
    serde_json::from_str(json_str)
}

pub async fn fetch_heartbeats(
    key: &str,
    start: &str,
    end: &str,
) -> Result<HeartbeatResponse, Box<dyn Error>> {
    let url = format!("{BASE_URL}/api/v1/my/heartbeats");

    //curl 'https://hackatime.hackclub.com/api/v1/my/heartbeats?start_time=&end_time=' \
    // --header 'Authorization: Bearer YOUR_SECRET_TOKEN'
    let resp = reqwest::Client::new()
        .get(&url)
        .bearer_auth(key)
        .query(&[("start_time", start), ("end_time", end)])
        .send()
        .await?;

    let status = resp.status();
    if status == StatusCode::UNAUTHORIZED {
        return Err("401 Unauthorized: Invalid API Key".into());
    }
    if status == StatusCode::UNAUTHORIZED {
        return Err("401 Unauthorized: Invalid API Key".into());
    }

    let text = resp.text().await?;
    let data = parse_heartbeats(&text)?;
    Ok(data)
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
        let res = fetch_heartbeats(&key, "2026-03-20T00:00:00Z", "2026-03-20T23:59:59Z").await;
        assert!(res.is_ok(), "Live fetch error: {:?}", res.err());
    }
}
