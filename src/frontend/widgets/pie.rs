use ratatui::layout::Rect;
use ratatui::widgets::Block;
use ratatui::Frame;
use tui_piechart::{PieChart, PieSlice};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Paragraph;

// umm i found this random crate online
// if you wanna use it - https://crates.io/crates/tui-piechart

pub fn render_pie(frame: &mut Frame, area:Rect, title: &str, slices: Vec<PieSlice>){
    let chart = PieChart::new(slices)
        .block(Block::bordered().title(title))
        .show_legend(true);

    frame.render_widget(chart, area);
}
