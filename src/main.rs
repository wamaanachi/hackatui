pub mod cli;
pub mod frontend;
pub mod api;
pub mod config;

// so writing a tiny script to test it out 
// maybe change or delete this later

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
// yep we'll def need to change this file later
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Color;
use tui_piechart::PieSlice;

use frontend::widgets::{
    pie::render_pie,
    projects::render_fake_ahh_projects,
    today::render_today,
};

fn main() -> std::io::Result<()>{
    let mut terminal = ratatui::init();

    let result = || -> std::io::Result<()>{
        loop {
            terminal.draw(|frame| {
                let slices = vec![
                    PieSlice::new("lath", 42.0, Color::Blue), // lets see of this works
                    PieSlice::new("rust", 22.0, Color::Yellow),
                ];

                render_pie(frame, frame.area(), "Cool Languages", slices);
            })?;
            if let Event::Key(Key) = event::read()? {
                if Key.code == KeyCode::Char('x'){
                    break;
                }
            }
        }
        Ok(())
    }();

    ratatui::restore();
    result
}