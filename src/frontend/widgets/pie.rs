use ratatui::layout::Rect;
use ratatui::widgets::Block;
use ratatui::Frame;
use tui_piechart::{PieChart, PieSlice};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Paragraph;

// umm i found this random crate online
// if you wanna use it - https://crates.io/crates/tui-piechart

pub fn render_pie(frame: &mut Frame, area:Rect, title: &str, slices: Vec<PieSlice>, legend: &str,){

    let block = Block::bordered().title(title);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(inner);

    let chart = PieChart::new(slices)
        .show_legend(false);

    frame.render_widget(chart, rows[0]);
    frame.render_widget(Paragraph::new(legend), rows[1]);
}
