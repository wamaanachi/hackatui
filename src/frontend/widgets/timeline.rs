// i lowk kinda rushed throught this file

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
// bruh i hv literraly copy pasted these like a milltion time

use crate::api::models::DashboardSummary;

pub fn render_timeline(frame: &mut Frame, area: Rect, summary: &DashboardSummary) {
    // Dynamic project names from real API summary
    let proj1 = summary
        .projects
        .first()
        .map(|p| p.name.as_str())
        .unwrap_or("Project 1");
    let proj2 = summary
        .projects
        .get(1)
        .map(|p| p.name.as_str())
        .unwrap_or("Project 2");

    let weekly = [
        ("Week 1", 220, 140),
        ("Week 2", 250, 20),
        ("Week 3", 10, 0),
        ("Week 4", 420, 210),
    ];

    let mut rows = vec![];

    for (week, time1, time2) in weekly {
        let total = time1 + time2;

        // found how to mkae this typa thing in docs
        rows.push(Line::from(vec![
            Span::raw(format!("{week:<7} ")),
            Span::styled(
                "█".repeat((time1 / 30) as usize),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(
                "█".repeat((time2 / 30) as usize),
                Style::default().fg(Color::Magenta),
            ),
            Span::raw(format!(" {}h {}m", total / 60, total % 60)),
        ]));
    }

    rows.push(Line::from(""));
    rows.push(Line::from(vec![
        Span::styled(format!("{proj1}  "), Style::default().fg(Color::Cyan)),
        Span::styled(format!(" {proj2}"), Style::default().fg(Color::Magenta)),
    ]));

    let chart = Paragraph::new(rows).block(Block::bordered().title("Project Timeline"));

    frame.render_widget(chart, area);
}
