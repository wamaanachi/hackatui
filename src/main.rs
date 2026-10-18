use clap::Parser;
use crate::cli::cli::CliArgs;

pub mod cli;
pub mod frontend;
pub mod api;
pub mod config;

fn main() {
    let args: CliArgs = CliArgs::parse();

    if !args.is_empty() {
        todo!(
            "Call the cli parser"
        )
    } else {
        todo!(
            "Do the regular tui?"
        )
    }
}
