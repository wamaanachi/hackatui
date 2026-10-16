// so tis is just a test file
// will i delete this later? idk
// its for the progress.rs file btw

use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Gauge};
use ratatui::Frame;

pub fn render_fake_ahh_projects(frame: &mut Frame, area:  Rect){
    let block = Block::bordered().title("Project Durations");
    let inner = block.inner(area);

    frame.render_widget(block, area);

    let projects = [
        ("von", "4h 20m", 100),
        ("on", "3h 00m", 67),
        ("n", "1h 30m", 33),
    ];

    for (i, (name, duration, percent)) in projects.iter().enumerate(){
        if i >= inner.height as usize {
            break;
        }

        let row = Rect::new(inner.x, inner.y + i as u16, inner.width, 1);

        let bar = Gauge::default()
            .gauge_style(Style::default().fg(Color::Blue))
            .label(format!("{name}: {duration}"))
            .percent(*percent);

            frame.render_widget(bar, row)
    }
}
