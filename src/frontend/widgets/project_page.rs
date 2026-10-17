// so this is the other page you see on hackatime - https://hackatime.hackclub.com/my/projects
// this gonna have similar type of tiny cards for each project

use ratatui::layout::Rect;
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};
use ratatui::Frame;

pub fn render_project_card(frame: &mut Frame, area: Rect, name: &str, duration: &str, description: &str, language: &str){
    let block = Block::bordered()
        .title(name)
        .border_type(BorderType::Rounded);

    let description = if description.is_empty(){
        "No description"
    }
    else {
        description
    };

    // wait i should hv placed the description first lol

    let text = format!("Coding time: {duration}\n\n{description}\n\nLanguages: {language}");

    let card = Paragraph::new(text)
        .block(block)
        .wrap(Wrap {trim:true});

        frame.render_widget(card, area);
}