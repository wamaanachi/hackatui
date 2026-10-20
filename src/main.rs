pub mod api;
pub mod cli;
pub mod config;
pub mod frontend;

use crate::cli::cli::CliArgs;
use crate::frontend::tui_stuff;
use clap::Parser;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: CliArgs = CliArgs::parse();

    let key = crate::config::load_key().expect("Failed to load API key from config!");

    // If CLI flags were given, fetch and print to stdout instead of starting TUI
    if !args.is_empty() {
        let summary = crate::api::fetch_dashboard(&key, Some("2026-10-01"), None).await?;
        crate::cli::parse(args, &summary);
    } else {
        println!("Fetching dashboard data from HackaTime...");
        let summary = crate::api::fetch_dashboard(&key, Some("2026-10-01"), None).await?;
        tui_stuff(&summary);
    }

    Ok(())
}
