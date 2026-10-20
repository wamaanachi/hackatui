use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Paragraph, Sparkline};

use crate::api::models::DashboardSummary;

pub fn render_project_details(frame: &mut Frame, area: Rect, summary: &DashboardSummary) {
    let top_project = summary.projects.first();
    let title = top_project
        .map(|p| p.name.as_str())
        .unwrap_or("No Projects");

    let block = Block::bordered().title(title);
    // ratatui is lowk good

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let sections = Layout::vertical([Constraint::Length(5), Constraint::Min(0)]).split(inner);

    let secs = top_project.map(|p| p.total_seconds as u64).unwrap_or(0);
    let percent = top_project.map(|p| p.percent).unwrap_or(0.0);
    let top_lang = summary
        .languages
        .first()
        .map(|l| l.name.as_str())
        .unwrap_or("None");

    let text_content = format!(
        "Total time:     {}h {}m ({:.1}%)\n\
         Streak:         {} days\n\
         Top language:   {}\n\
         \n\
         Daily activity:",
        secs / 3600,
        (secs % 3600) / 60,
        percent,
        summary.streak,
        top_lang,
    );

    let text = Paragraph::new(text_content);

    frame.render_widget(text, sections[0]);

    // Daily activity bucket sparkline (kept as placeholder until heartbeats timeline is wired in)
    let activity = [40, 20, 60, 30, 50, 40, 120, 140];

    let chart = Sparkline::default()
        .data(&activity)
        .style(Style::default().fg(Color::Cyan));

    frame.render_widget(chart, sections[1]);
}
