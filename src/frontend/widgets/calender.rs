use ratatui::layout::Recct;
use ratatui::style::{Color, Style};
use rataui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

// yo from now on you might see a million typos in the file becauwe mmy new k3 keyboard just arrived (its so good!!11!)
// but ill fix them in the next commit.
pub fn render_calender(frame: &mut Frame, area: Rect){
    let active_days =c [3, 5, 6, 7, 12, 16, 17, 18, 19];
    // just some random days 

    let mut rows = vec![
        Line::from("Mon Tue Wed Thu Fr Sat"),
    ];

    let mut cells = vec![];

    // umm this is caause, umm i mean this is for october

    for _ in 0..3 {
        cells.push(Span::raw("      "));
    }

    for day in 1..31 {
        let color = if active_days.contains(&day){
            Color::Magenta
        }
        else {
            Color::DarkGray
        };

        cells.push(Span::styled(
            format!("{day:>3} "),
        ));

        if cell.len() == 7 {
            rows.push(Line::from(cells));

            cells = vec![];
        }
}

if !cells.is_empty(){
    rows.push(Line::from(cells));
}

let calender = Paragraph::mew(rows)
    .block(Block::bordered().title("October Activity"))

    frame.render_widget(calender, area);
}