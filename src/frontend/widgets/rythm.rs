// this will be the rythm chart that you see in hackatime dash
// i lowk couldnt find any good doc forthis :(

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use crate::api::models::DashboardSummary;

pub fn render_rythm(frame: &mut Frame, area: Rect, _summary: &DashboardSummary) {
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    let minutes = [
        0, 0, 5, 45, 35, 10, 0, 0, 25, 55, 45, 10, 40, 20, 0, 0, 20, 45, 35, 20, 10, 25, 20,
        30,
        // now they are 24 for the chart thnk yk?
    ];

    let mut rows = vec![];

    let mut hours = String::from("     ");

    for hour in 0..24 {
        hours.push_str(&format!("{hour:02} ")); // um this so the hours are dusplayed like 06 instead 6 looks cool lol
    }

    rows.push(Line::from(hours));
    // lollll
    for day in days {
        let mut cells = vec![Span::raw(format!("{day}  "))];
        //bruhh
        // handwriting these examples is boring

        for value in minutes {
            let color = if value == 0 {
                Color::DarkGray
            } else if value < 30 {
                Color::Rgb(100, 45, 70)
                //yo replacing the colo r names with rgb i got in a picker thingy cause most colors dont work lol // Color::DarkPink // idk man
            } else {
                Color::Rgb(235, 105, 145)
            };

            // oh yeah this part was mentioned in te docs

            cells.push(Span::styled("  ", Style::default().bg(color)));
            cells.push(Span::raw(" "));
        }
        rows.push(Line::from(cells));

        rows.push(Line::from(vec![]));
    }

    let widget = Paragraph::new(rows).block(Block::bordered().title("The Rythm"));

    frame.render_widget(widget, area);
}
