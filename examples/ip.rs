// examples/ip.rs
use hackatui::api::fetch_heartbeats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = hackatui::config::load_key()?;
    let body = fetch_heartbeats(&key, "2026-10-05T00:00:00Z", "2026-10-11T23:59:59Z").await?;
    println!("{:?}", &body.heartbeats[..10.min(body.heartbeats.len())]);
    Ok(())
}
