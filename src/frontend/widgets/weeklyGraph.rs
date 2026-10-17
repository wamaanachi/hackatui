// so i got confused here for a bit so i read - https://docs.rs/ratatui/latest/ratatui/widgets/struct.BarChart.html
// but ye ill obv change the code later to suit this

use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{BarChart, Block};
use ratatui::Frame;

pub fn render_weekly(frame: &mut Frame, area: Rect){
let chart = BarChart::default()
    .block(Block::bordered().title("WeeklyGraph"))
    .bar_width(4) // umm idk ill change it ig dpeending on how the project looks altogether with the widgets
    .bar_gap(1)
    .group_gap(3)
    .bar_style(Style::new().cyan()) // no clue how this is gonna look
    .value_style(Style::new().black().on_cyan().bold())
    .label_style(Style::new().white())
    .data(&[("Mon", 2), ("Tue", 3), ("Wed", 1), ("Thu", 1), ("Fri", 5), ("Sat", 2), ("Sun", 8)]);

    frame.render_widget(chart, area)
}