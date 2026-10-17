// so tis is just a test file
// will i delete this later? idk
// its for the progress.rs file btw

use ratatui::widgets::{Block, Gauge, Paragraph};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::Frame;

pub fn render_fake_ahh_projects(frame: &mut Frame, area:  Rect, projects: &[(&str, &str, u16)],){
    let block = Block::bordered().title("Project Durations");
    let inner = block.inner(area);

    frame.render_widget(block, area);

    // let projects = [
    //     ("hackaTui", "4h 20m", 100),
    //     ("prophesizer", "3h 00m", 67),
    //     ("lankat", "1h 30m", 33),
    // ];

    // will take this block to main.rs as i dont want it to be hardcodrd

                if inner.width < 25 || inner.height < 1 {
                frame.render_widget(Paragraph::new("Needs more space"), inner);
                return;
            }

            if projects.is_empty(){
                frame.render_widget(Paragraph::new("No time recorded yet"), inner);
                return;
            }

    for (i, (name, duration, percent)) in projects.iter().enumerate(){
        if i >= inner.height as usize {
            break;
        }

        let row = Rect::new(inner.x, inner.y + i as u16, inner.width, 1);

        let bar = Gauge::default()
            .gauge_style(Style::default().fg(Color::Blue))
            .label(format!("{name}: {duration}"))
            .percent((*percent).min(100));

            frame.render_widget(bar, row)

            // so this gonna be like error handling and stuff
}
}