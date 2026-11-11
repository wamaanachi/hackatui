use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use tui_piechart::{PieChart, PieSlice};

use crate::api::models::Share;

const PALETTE: [Color; 6] = [
    Color::Red,
    Color::Blue,
    Color::Green,
    Color::Yellow,
    Color::Magenta,
    Color::Cyan,
];

/// Draws a pie chart with a legend for a breakdown list from `DashboardSummary`.
///
/// Each entry's color is taken from `PALETTE` by position, so the slice and
/// its legend line always match. `percent` comes from the data, not from
/// this function.
pub fn render_pie(frame: &mut Frame, area: Rect, title: &str, shares: &[Share]) {
    let block = Block::bordered().title(title);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if shares.is_empty() {
        frame.render_widget(Paragraph::new("No activity yet"), inner);
        return;
    }

    let slices: Vec<PieSlice> = shares
        .iter()
        .enumerate()
        .map(|(i, s)| PieSlice::new(s.name.as_str(), s.total_seconds, PALETTE[i % PALETTE.len()]))
        .collect();

    let legend: Vec<Line> = shares
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let secs = s.total_seconds as u64;
            Line::from(vec![
                Span::styled("■ ", Style::default().fg(PALETTE[i % PALETTE.len()])),
                Span::raw(format!(
                    "{}: {}h {}m ({:.1}%)",
                    s.name,
                    secs / 3600,
                    (secs % 3600) / 60,
                    s.percent,
                )),
            ])
        })
        .collect();

    let legend_height = (shares.len() as u16).min(inner.height);
    let rows =
        Layout::vertical([Constraint::Min(0), Constraint::Length(legend_height)]).split(inner);

    let chart = PieChart::new(slices).show_legend(false);

    frame.render_widget(chart, rows[0]);
    frame.render_widget(Paragraph::new(legend), rows[1]);
}
