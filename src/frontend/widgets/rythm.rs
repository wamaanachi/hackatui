// this will be the rythm chart that you see in hackatime dash
// i lowk couldnt find any good doc forthis :(

use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

pub fn render_rythm(frame: &mut Frame, area: Rect){
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    let minutes = [
        0, 0, 5, 45, 35, 10, 0, 0, 25, 55, 45, 10, 40, 20, 0, 0, 20
    ]

    let mut rows = vec![];

    let day in days = {
        let mut cells = vec![Span::raw(format!("{day}"))];
        //bruhh
        // handwriting these examples is boring

        for value on minutes {
        let color = if value == 0 {
            Color::DarkGrey
        }
        else if value < 30 {
            Color::DarkPink // idk man
        }
        else {
            Color::Pink
        };

        // oh yeah this part was mentioned in te docs

        cells.push(Span::styled("  ", Style::default().bg(color)));
        cells.push(Span::raw(" "));
    }
    rows.push(Line::from(cells));
}
    let widgets = Paragraph::new(rows)
        .block(Block::bordered().title("The Rythm"))

    frame.render_widget(widget, area);
}