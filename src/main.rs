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
use ratatui::widgets::Paragraph;

// gonna test out all of these now
use frontend::widgets::{
    pie::render_pie,
    progress::render_progress,
    projects::render_fake_ahh_projects,
    today::render_today,
    weeklyGraph::render_weekly,
};

// ok so im gonna display them 1 by 1 for now cause i aint making the dashboard :hehehehheheeheheheheheh:
fn main() -> std::io::Result<()>{
    let mut terminal = ratatui::init();
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
                    '1' => render_today(frame, areas[1], "8h 20m"),

                    '2' => render_progress(frame, areas[1], "8h 20m", 67),

                    '3' => render_fake_ahh_projects(frame, areas[1]),

                    '4' => render_weekly(frame, areas[1]),

                    '5' => {
                let slices = vec![
                    PieSlice::new("lath", 42.0, Color::Blue), // lets see of this works
                    PieSlice::new("rust", 22.0, Color::Yellow),
                ];

                  render_pie(frame, areas[1], "Cool Languages", slices, "Blue: lath | Yellow: rust",)
                    }
                        
                    '6' => {
                        let editors = vec![
                            PieSlice::new("Veonim", 480.0, Color::Blue),
                            PieSlice::new("emacs", 72.0, Color::Yellow),
                                        ];

                        render_pie(frame, areas[1], "Editors", editors, "Blue: Veonim | Yellow: Emacs");
                    }
                    

                    _ => {}
                }
            })?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press{
                    match key.code{
                        KeyCode::Char('x') => break,

                        KeyCode::Char(number @ '1'..='6') => { // lol i coundnt figure out how to loop
                            selected = number;
                        }

                        _=> {}
                    }
                }
            }
        }
        Ok(())
    }();

    ratatui::restore();
    result
}