// this module is for the frontend

pub mod widgets;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
// yep we'll def need to change this file later
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Paragraph;

use crate::api::models::DashboardSummary;

// gonna test out all of these now
use crate::frontend::widgets::{
    calender::render_calender, pie::render_pie, progress::render_progress,
    project_details::render_project_details, project_page::render_project_card,
    projects::render_fake_ahh_projects, rythm::render_rythm, summary::render_summary,
    timeline::render_timeline, weekly_graph::render_weekly,
};

pub(crate) fn tui_stuff(summary: &DashboardSummary) {
    let mut terminal = ratatui::init();

    let mut selected = '1';

    let result = || -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| {
                // umm yea this code shouldnt be in main lol
                let areas = Layout::vertical([
                    Constraint::Length(2),
                    Constraint::Min(0),
                ])
                .split(frame.area());

                let menu = Paragraph::new(
                    "1 Summary | 2 Progress | 3 Projects | 4 Weekly | 5 Languages | 6 Editors | 7 Rythm | 8 Details | 9 Cards | 0 Calendar | t Timeline | q Exit",
                );

                frame.render_widget(menu, areas[0]);

                match selected {
                    // '1' => render_message(frame, areas[1], "No Activity", "No coding activity in this period"),

                    '1' => render_summary(frame, areas[1], summary),

                    '2' => render_progress(frame, areas[1], summary, 240),

                    '3' => render_fake_ahh_projects(frame, areas[1], &summary.projects),

                    '4' => render_weekly(frame, areas[1], summary),

                    '5' => render_pie(frame, areas[1], "Languages", &summary.languages),

                    '6' => render_pie(frame, areas[1], "Editors", &summary.editors),

                    '7' => render_rythm(frame, areas[1], summary),
                    // i put the example data in the rythm file

                    '8' => render_project_details(frame, areas[1], summary),

                    '9' => {
                        let rows = Layout::vertical(vec![
                            Constraint::Length(8);
                            summary.projects.len().max(1)
                        ])
                        .split(areas[1]);

                        for (i, project) in summary.projects.iter().enumerate() {
                            render_project_card(frame, rows[i], project);
                        }
                    }

                    '0' => render_calender(frame, areas[1], summary),

                    't' => render_timeline(frame, areas[1], summary),
                    _ => {}
                }
            })?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('x') => break,

                        KeyCode::Char(number @ '1'..='9') => {
                            // lol i coundnt figure out how to loop
                            selected = number;
                        }

                        KeyCode::Char('0') => {
                            selected = '0';
                        }

                        KeyCode::Char('t') => {
                            selected = 't';
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
