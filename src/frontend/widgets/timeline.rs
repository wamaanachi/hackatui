// i lowk kinda rushed throught this file

use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
// bruh i hv literraly copy pasted these like a milltion time 

pub fn render_timeline(frame: &mut Frame, area: Rect, weekly: &[(String, Vec<u64>)], names: &[&str]){
    let mut rows = vec![];

    let colors = [Color::Cyan, Color::Magenta, Color::Blue, Color::Yellow, Color::Green];
    for (week, projects) in weekly {
        let total: u64 = projects.iter().sum();
        let mut cells = vec![Span::raw(format!("{week} "))];
        for (i, minutes) in projects.iter().enumerate() {
            cells.push(Span::styled("█".repeat((minutes / 30).min(area.width as u64) as usize),
                Style::default().fg(colors[i % colors.len()])));
        }
        cells.push(Span::raw(format!(" {}h {}m", total / 60, total % 60)));
        rows.push(Line::from(cells));
    }
    rows.push(Line::from(""));
    rows.push(Line::from(names.iter().enumerate().map(|(i, name)|
        Span::styled(format!("{name}  "), Style::default().fg(colors[i % colors.len()]))
    ).collect::<Vec<_>>()));

    let chart = Paragraph::new(rows)
        .block(Block::bordered().title("Project Timeline (4 weeks, UTC)"));

        frame.render_widget(chart, area);
}