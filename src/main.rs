pub mod cli;
pub mod frontend;
pub mod api;
pub mod config;


use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
// yep we'll def need to change this file later
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Color;
use tui_piechart::PieSlice;
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
};


use clap::Parser;
use crate::cli::cli::CliArgs;
use crate::cli::parse;

fn main() {
    let args: CliArgs = CliArgs::parse();

    // I made it so that the tui stuff only runs when no cli args passed
    if !args.is_empty() {
        // todo!(
        //     "Call the cli parser, then sys.exit()"
        // )

        // (just ignores it for now)
        let ignore: bool = true;
    }

    let mut terminal = ratatui::init();

    let seconds: u64 = 9000;
    let minutes = seconds / 60;
    let time = format!("{}h {}m", minutes / 60, minutes % 60);
    let mut selected = '1';

    let result = || -> std::io::Result<()>{
        loop {
            terminal.draw(|frame| {
                // umm yea this code shouldnt be in main lol
                let areas = Layout::vertical([
                    Constraint::Length(2),
                    Constraint::Min(0),
                ])
                    .split(frame.area());

                let menu = Paragraph::new(
                    "1 Today | 2 Progress | 3 Projects | 4 Weekly | 5 Pie | q Exit",
                );

                frame.render_widget(menu, areas[0]);

                match selected {
                    '1' => render_message(frame, areas[1], "No Activity", "No coding activity in this period"),

                    '2' => render_progress(frame, areas[1], minutes, 240),

                    '3' => {
                        let projects = [
                            ("hackaTui", "4h 20m", 100),
                            ("prophesizer", "3h 00m", 67),
                            ("lankat", "1h 30m", 33),
                        ];
                        render_fake_ahh_projects(frame, areas[1], &projects)
                    },

                    '4' => render_weekly(frame, areas[1]),

                    '5' => {
                        let slices = vec![
                            PieSlice::new("lath", 42.0, Color::Blue), // lets see of this works
                            PieSlice::new("rust", 22.0, Color::Yellow),
                        ];

                        render_pie(frame, areas[1], "Cool Languages", slices, "Blue: lath | Yellow: rust", )
                    }

                    '6' => {
                        let editors = vec![
                            PieSlice::new("Veonim", 480.0, Color::Blue),
                            PieSlice::new("emacs", 72.0, Color::Yellow),
                        ];

                        render_pie(frame, areas[1], "Editors", editors, "Blue: Veonim | Yellow: Emacs");
                    }

                    '7' => render_rythm(frame, areas[1]),
                    // i put the example data in the rythm file

                    '8' => render_project_details(frame, areas[1]),

                    '9' => {
                        let projects = [
                            ("hackatui", "8h 14m", "Hackatime in terminal", "Rust"),
                            ("lath", "3h 42m", "Cool programming language", "Rust"),
                            ("timetable", "7h 25m", "Timetable generator", "Javascript"),
                        ];

                        let rows = Layout::vertical(vec![Constraint::Length(8); projects.len()],
                        ).split(areas[1]);

                        for (i, (name, time, description, language)) in projects.iter().enumerate() {
                            render_project_card(frame, rows[i], name, time, description, language, );
                        }
                    }

                    '0' => render_calender(frame, areas[1]),
                    _ => {}
                }
            })?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('x') => break,

                        KeyCode::Char(number @ '1'..='9') => { // lol i coundnt figure out how to loop
                            selected = number;
                        }

                        KeyCode::Char('0') => {
                            selected = '0';
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }();

    ratatui::restore();
    result.expect("picnicing");
}
