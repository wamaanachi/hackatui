use ratatui::layout::Rect;
use ratatui::widgets::Block;
use ratatui::Frame;
use tui_piechart::{PieChart, PieSlice};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Paragraph;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

// umm i found this random crate online
// if you wanna use it - https://crates.io/crates/tui-piechart

pub fn render_pie(frame: &mut Frame, area:Rect, title: &str, data: &[(&str, u64, Color)],){

    let block = Block::bordered().title(title);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut total = 0;
    for (_, seconds, _) in data {
        total += seconds;
    }

    if total == 0 {
        frame.render_widget(Paragraph::new("No activity yet"), inner);
        return;
    }

    let mut slices = vec![];
    let mut legend = vec![];

    for (name, seconds, color) in data {
        slices.push(PieSlice::new(*name, *seconds as f64, *color));

        let percent = *seconds as f64 / total as f64 * 100.0;
        let minutes = seconds / 60;
        
        // so it actually shows the numbers, yk the percemtage and umm total hourw

        legend.push(Line::from(vec![
            Span::raw(format!(
                "{name}: {}h {}m ({percent:.1}%)",
                minutes / 60,
                minutes % 60,
            )),
        ]));
    }

    let rows = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(data.len().min(inner.height as usize) as u16),
    ])
    .split(inner);

    let chart = PieChart::new(slices)
        .show_legend(false);

    frame.render_widget(chart, rows[0]);
    frame.render_widget(Paragraph::new(legend), rows[1]);
}    