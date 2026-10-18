use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

// yo from now on you might see a million typos in the file becauwe mmy new k3 keyboard just arrived (its so good!!11!)
// but ill fix them in the next commit.
pub fn render_calender(frame: &mut Frame, area: Rect){
    let active_days = [3, 5, 6, 7, 12, 16, 17, 18, 19];
    // just some random days 

    let today = 18;

    let mut streak = 0;
    let mut longest = 0;
    let mut active_count = 0;

    for day in 1..=today {
        if active_days.contains(&day){
    streak +=1;
    active_count +=1;

    if streak > longest {
        longest = streak;
    }
        }
        else {
            streak = 0;
        }
    }

    let mut current = 0;
    let mut day = today;

    if !active_days.contains(&day){
        day -= 1;
    }

    while day > 0 && active_days.contains(&day){
        current += 1;
        day -= 1;
    }
    let mut rows = vec![
        Line::from("Mon Tue Wed Thu Fri Sat Sun"),
    ];

    let mut cells = vec![];

    // umm this is caause, umm i mean this is for october

    for _ in 0..3 {
        cells.push(Span::raw("    "));
    }

    for day in 1..=31 {
        let color = if day <= today && active_days.contains(&day){
            Color::Magenta
        }
        else {
            Color::DarkGray
        };

        cells.push(Span::styled(
            format!("{day:>3} "),
            Style::default().fg(color),
        ));

        if cells.len() == 7 {
            rows.push(Line::from(cells));

            cells = vec![];
        }
}

if !cells.is_empty(){
    rows.push(Line::from(cells));
}

rows.push(Line::from(""));
rows.push(Line::from(format!("Current streak: {current} days")));
rows.push(Line::from(format!("Longest streak: {longest} days")));
rows.push(Line::from(format!("Active days: {active_count}")));

let calender = Paragraph::new(rows)
    .block(Block::bordered().title("October Activity"));

    frame.render_widget(calender, area);
}