// this module is for the frontend

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
// yep we'll def need to change this file later
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Color;
use ratatui::widgets::Paragraph;

// gonna test out all of these now
use crate::frontend::widgets::{
    calender::render_calender,
    pie::render_pie,
    progress::render_progress,
    project_details::render_project_details,
    project_page::render_project_card,
    projects::render_fake_ahh_projects,
    rythm::render_rythm,
    summary::render_summary,
    timeline::render_timeline,
    weeklyGraph::render_weekly,
};

pub mod widgets;