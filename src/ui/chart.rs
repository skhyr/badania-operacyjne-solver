use crate::point::Point;
use tui::{
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
};

pub fn get_chart_data(data: &Vec<Vec<Point>>) -> Vec<Vec<(f64, f64)>> {
    let datas = data
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|point| (point.0.into(), point.1.into()))
                .collect::<Vec<(f64, f64)>>()
        })
        .collect::<Vec<Vec<(f64, f64)>>>();
    datas
}

pub fn get_chart(chart_data: &Vec<Vec<(f64, f64)>>) -> Chart {
    let datasets: Vec<tui::widgets::Dataset> = chart_data
        .into_iter()
        .enumerate()
        .map(|(i, set)| {
            let index: i32 = i.try_into().unwrap();
            Dataset::default()
                .name("Equation ".to_string() + &index.to_string())
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(COLORS[i]))
                .graph_type(GraphType::Line)
                .data(&set)
        })
        .collect();

    Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Chart",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("x")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 200.0])
                .labels(vec![]),
        )
        .y_axis(
            Axis::default()
                .title("y")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 200.0])
                .labels(vec![]),
        )
}

static COLORS: [Color; 6] = [
    Color::Red,
    Color::Green,
    Color::Blue,
    Color::Yellow,
    Color::Magenta,
    Color::Cyan,
];
