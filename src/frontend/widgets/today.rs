// so rn im making like imcomplete widgets (the boxes you see on hackatime dashboard)
// will wire (put inside functions) later when the api stuff is done

// im using ratatui official docs, when im stuck n stuff

use ratatui::widgets::{Block, BorderType};
use ratatui::layout::Rect;
use ratatui::Frame;

fn render_today(frame: &mut Frame, area: Rect){

    let today_box = Block::bordered()
    .title("Today's Time")
    .border_type(BorderType::Rounded);

    frame.render_widget(today_box, area);
}