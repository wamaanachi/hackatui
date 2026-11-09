use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Paragraph, Sparkline};
use ratatui::Frame;

pub fn render_project_details(frame: &mut Frame, area: Rect, name: &str, total: &str, week: &str, languages: &str, activity: &[u64]){

    let block = Block::bordered().title(name);
        // ratatui is lowk good

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let sections = Layout::vertical([
        Constraint::Length(5),
        Constraint::Min(0),
    ])
    .split(inner);

    let text = Paragraph::new(format!(
        "Loaded range: {total}\nThis week: {week}\nLanguages: {languages}\n\nDaily activity (minutes, UTC):"
    ));

    frame.render_widget(text, sections[0]);


    let chart = Sparkline::default()
        .data(activity)
        .style(Style::default().fg(Color::Cyan));

    frame.render_widget(chart, sections[1]);
}