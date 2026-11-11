// i lowk kinda rushed throught this file

use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
// bruh i hv literraly copy pasted these like a milltion time 

pub fn render_timeline(frame: &mut Frame, area: Rect){
    let weekly = [
        ("Week 1", 220, 140),
        ("Week 2", 250, 20),
        ("Week 3", 10, 0),
        ("Week 4", 420, 210),
    ];

    let mut rows = vec![];

    for (week, hackatui, lath) in weekly {
        let total = hackatui +  lath;

        // found how to mkae this typa thing in docs
        rows.push(Line::from(vec![
            Span::raw(format!("{week} ")),

            Span::styled(
                "█".repeat(hackatui / 30),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(
                "█".repeat(lath / 30),
                Style::default().fg(Color::Magenta),
            ),
            Span::raw(format!(" {}h {}m", total / 60, total % 60)),
        ]));
    }

    rows.push(Line::from(""));
    rows.push(Line::from(vec![
        Span::styled("hackatui  ", Style::default().fg(Color::Cyan)),
        Span::styled(" lath", Style::default().fg(Color::Magenta)),
    ]));

    let chart = Paragraph::new(rows)
        .block(Block::bordered().title("Project Timeline"));

        frame.render_widget(chart, area);
}