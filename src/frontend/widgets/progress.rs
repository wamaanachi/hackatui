// so this file will contain that project durations box so you on the dashboard

// rn it is 1 bar, ill change it later
// and this too is uncomplete as obv the api stuff is not done

use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Gauge};
use ratatui::Frame;

fn render(frame: &mut Frame){

    let progress = 0.67;

    let gauge = Gauge::default()
        .block(Block::bordered().title("Download Progress"))
        .gauge_style(Style::new().cyan().bold())
        .ratio(progress);

    frame.render(gauge, frame.area());
}