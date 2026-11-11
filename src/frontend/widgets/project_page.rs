// so this is the other page you see on hackatime - https://hackatime.hackclub.com/my/projects
// this gonna have similar type of tiny cards for each project

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};

use crate::api::models::Share;

pub fn render_project_card(frame: &mut Frame, area: Rect, project: &Share) {
    let block = Block::bordered()
        .title(project.name.as_str())
        .border_type(BorderType::Rounded);

    let secs = project.total_seconds as u64;
    let duration = format!(
        "{}h {}m ({:.1}%)",
        secs / 3600,
        (secs % 3600) / 60,
        project.percent
    );

    // wait i should hv placed the description first lol

    let text = format!(
        "Coding time: {duration}\n\nTracked via Hackatime\n\nShare of total: {:.1}%",
        project.percent
    );

    let card = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

    frame.render_widget(card, area);
}
