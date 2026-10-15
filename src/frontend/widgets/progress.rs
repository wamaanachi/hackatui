// so this file will contain that project durations box so you on the dashboard

// rn it is 1 bar, ill change it later
// and this too is uncomplete as obv the api stuff is not done

use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Gauge};
use ratatui::Frame;

pub fn render_progress(frame: &mut Frame, area:Rect, duration: &str, percent: u16){

    let gauge = Gauge::default()
        .block(Block::bordered().title("Progress Time"))
        .gauge_style(Style::new().cyan().bold())
        .label(duration)
        .percent(percent.min(100));

    frame.render_widget(gauge, area);
}