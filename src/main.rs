pub mod cli;
pub mod frontend;
pub mod api;
pub mod config;


use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
// yep we'll def need to change this file later
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Color;
use ratatui::widgets::Paragraph;
use frontend::widgets::message::render_message;

// gonna test out all of these now
use frontend::widgets::{
    pie::render_pie,
    progress::render_progress,
    projects::render_fake_ahh_projects,
    today::render_today,
    weeklyGraph::render_weekly,
    rythm::render_rythm,
    project_details::render_project_details,
    project_page::render_project_card,
    calender::render_calender,
    summary::render_summary,
    timeline::render_timeline,
};


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
        let ignore: bool = true;
    } else {
        tui_stuff()
    }
}
