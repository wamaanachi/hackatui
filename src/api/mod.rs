// This module is for all the api related stuff

//Let's get started with fetching some basic info first.
//

use std::error::Error;

pub mod models;
const BASE_URL: &str = "https://hackatime.hackclub.com";

pub async fn fetch_heartbeats(key: &str, start: &str, end: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("{BASE_URL}/api/v1/my/heartbeats");

    //curl 'https://hackatime.hackclub.com/api/v1/my/heartbeats?start_time=&end_time=' \
    // --header 'Authorization: Bearer YOUR_SECRET_TOKEN'
    let resp = reqwest::Client::new()
        .get(&url)
        .bearer_auth(key)
        .query(&[("start_time", start), ("end_time", end)])
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(format!("request failed with status: {}", resp.status()).into());
    }

    Ok(resp.text().await?)
}
