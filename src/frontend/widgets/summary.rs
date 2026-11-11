use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Paragraph};

use crate::api::models::DashboardSummary;

pub fn render_summary(frame: &mut Frame, area: Rect, summary: &DashboardSummary) {
    // these are the 6 boxes you see on top of the hackatime dash, idk if the api has all of these

    let titles = [
        "Total Time",
        "Top Project",
        "Top Language",
        "Top OS",
        "Top Editor",
        "Top Category",
    ];

    let total_secs = summary.total_seconds as u64;
    let total_time = format!("{}h {}m", total_secs / 3600, (total_secs % 3600) / 60);

    let top_project = summary
        .projects
        .first()
        .map(|p| p.name.as_str())
        .unwrap_or("None");
    let top_language = summary
        .languages
        .first()
        .map(|l| l.name.as_str())
        .unwrap_or("None");
    let top_os = summary
        .operating_systems
        .first()
        .map(|o| o.name.as_str())
        .unwrap_or("None");
    let top_editor = summary
        .editors
        .first()
        .map(|e| e.name.as_str())
        .unwrap_or("None");
    let top_category = "Coding"; // Standard default category on Hackatime

    let values = [
        total_time.as_str(),
        top_project,
        top_language,
        top_os,
        top_editor,
        top_category,
    ];

    let area = Rect::new(area.x, area.y, area.width, area.height.min(3));
    // it looked very weid beefore i wrote this lol

    let cards = Layout::horizontal([Constraint::Ratio(1, 6); 6]).split(area);

    for i in 0..6 {
        let color = Color::DarkGray;

        let block = Block::bordered()
            .title(titles[i])
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(color));

        let card = Paragraph::new(values[i]).block(block);

        frame.render_widget(card, cards[i]);
    }
}
