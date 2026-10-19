pub mod cli;
pub mod frontend;
pub mod api;
pub mod config;


use clap::Parser;
use crate::cli::cli::CliArgs;
use crate::cli::parse;

use crate::frontend::tui_stuff;

fn main() {
    let args: CliArgs = CliArgs::parse();

    // I made it so that the tui stuff only runs when no cli args passed
    if !args.is_empty() {
        // parse(args);

        // (just ignores it for now)
        let _ignore: bool = true;
    } else {
        tui_stuff()
    }
}
