// so rn im making like imcomplete widgets (the boxes you see on hackatime dashboard)
// will wire (put inside functions) later when the api stuff is done

// im using ratatui official docs, when im stuck n stuff

use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::{Block, BorderType, Paragraph};

use crate::api::models::DashboardSummary;

pub fn render_today(frame: &mut Frame, area: Rect, summary: &DashboardSummary) {
    let secs = summary.total_seconds as u64;
    let time = format!("{}h {}m", secs / 3600, (secs % 3600) / 60);

    let today_box = Block::bordered()
        .title("Today's Time")
        .border_type(BorderType::Rounded);

    let today_widget = Paragraph::new(time)
        .alignment(Alignment::Left)
        .block(today_box);

    frame.render_widget(today_widget, area);
}
