use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Paragraph, Sparkline};
use ratatui::Frame;

pub fn render_project_details(frame: &mut Frame, area: Rect){

    let block = Block::bordered(),title("hackatui");
        // ratatui is lowk good

    let inner = blcok.inner(area);
    frame.render_widget(block, area);

    let sections = Layout::vertical([
        Constraint::Length(5),
        Constraint::Min(0),
    ])
    .split(inner)

    let text = Paragraph::new(
       "Total time:     14h 40m\n\
        This week:      8h 30m\n\
        Top language:   Rust\n\
        \n\
        Daily activity:",
    );

    frame.render_widget(text, sections[0]);

    let activity = [40, 20, 60, 30, 50, 40, 120, 140];

    let chart = Sparkline::default()
        .data(&activity)
        .style(Style::default().fg(Color::Cyan));

    frame.render_widget(chart, sections[1]);
}