use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};

pub fn render_message(frame: &mut Frame, area: Rect, title: &str, message: &str) {
    let block = Block::bordered()
        .title(title)
        .border_type(BorderType::Rounded);

    let widget = Paragraph::new(message)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(widget, area);
}
