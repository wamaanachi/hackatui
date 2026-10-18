// so this file will contain that project durations box so you on the dashboard

// rn it is 1 bar, ill change it later
// and this too is uncomplete as obv the api stuff is not done

use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Gauge};
use ratatui::Frame;

// so im going to tranform this file into a goal tracker thing i think

pub fn render_progress(frame: &mut Frame, area:Rect, minutes: u64, goal: u64,){

    let percent = if goal == 0 {
        0
    }
    else {
        (minutes as f64 / goal as f64 * 100.0).min(100.0) as u16
    };

    let remaining = goal.saturating_sub(minutes);

    let label = format!("{}h {}m / {}h {}m — {}m left", minutes / 60, minutes % 60, goal / 60, goal % 60, remaining,);
    // this is important to make the progress bar thingy

    let gauge = Gauge::default()
        .block(Block::bordered().title("Daily Goal"))
        .gauge_style(Style::new().cyan())
        .label(label)
        .percent(percent);

        let area = Rect::new(area.x, area.y, area.width, area.height.min(3));
    frame.render_widget(gauge, area);
}