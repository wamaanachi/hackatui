use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

// yo from now on you might see a million typos in the file becauwe mmy new k3 keyboard just arrived (its so good!!11!)
// but ill fix them in the next commit.
pub fn render_calender(frame: &mut Frame, area: Rect){

    // im gonna change it so colors would change depending on the time you codded that day 
    
    let activity = [
        (3, 10),
        (5, 210),
        (6, 20),
        (7, 410),
        (10, 25),
        (12, 80),
        (14, 30),
        (16, 42),
        (18, 140),
        (20, 240),
        (23, 150),
        (28, 88),
    ];
    // just some random days 

    let mut active_days = vec![];

    for (day, minutes) in activity {
        if minutes > 0 {
            active_days.push(day);
        }
    }

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

        let mut minutes = 0;

        for (date, total) in activity {
            if date == day {
                minutes = total;
            }
        }

        // you know what, i might change it so the shade of color depends on the exact time you coded
        // instead making a threshold for color 
        // oh yeah that'd so ccol ill def do it
        let color = if day > today || minutes == 0 {
            Color::DarkGray
        } 
        else if minutes < 60 {
            Color::Rgb(100, 45, 70)
        }
        else {
            Color::Rgb(235, 100, 145)
        };

        // yep will ddefinately change thos hardcoded rgbs
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