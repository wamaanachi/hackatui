use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::Frame;

pub fn render_summary(frame: &mut Frame, area: Rect, values: [&str; 6],){
    // these are the 6 boxes you see on top of the hackatime dash, idk if the api has all of these

    let titles = [
        "Total Time", "Top Project", "Top Language", "Top OS", "Top Editor", "Top Category",
    ];

    let area = Rect::new(area.x, area.y, area.width, area.height.min(3));
    // it looked very weid beefore i wrote this lol

    let cards = Layout::horizontal([
        Constraint::Ratio(1, 6);
        6
    ])
    .split(area);

    for i in 0..6 {

        let color = Color::DarkGray;

    let block = Block::bordered()
        .title(titles[i])
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(color));

        let card = Paragraph::new(values[i])
            .block(block);

        frame.render_widget(card, cards[i]);
}

}