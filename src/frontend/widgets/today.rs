// so rn im making like imcomplete widgets (the boxes you see on hackatime dashboard)
// will wire (put inside functions) later when the api stuff is done

// im using ratatui official docs, when im stuck n stuff

use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::layout::Rect;
use ratatui::Frame;

pub fn render_today(frame: &mut Frame, area: Rect, time: &str){

    let today_box = Block::bordered()
        .title("Today's Time")
        .border_type(BorderType::Rounded);

    let today_widget = Paragraph::new(time)
        .alignment(Alignment::left)
        .block(today_box);

    frame.render_widget(today_widget, area);
}