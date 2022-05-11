use crate::logic::get_points_range::get_points_range;
use crate::models::equation::Equation;
use crate::models::point::Point;
use crate::ui::colors::COLORS;
use tui::{
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
};

pub fn get_chart_data(data: &Vec<Equation>, max_x: f32, max_y: f32) -> Vec<Vec<(f64, f64)>> {
    data.into_iter()
        .map(|equation| {
            let res: Vec<(f64, f64)> = get_points_range(equation, max_x, max_y)
                .into_iter()
                .map(|point| (point.0.into(), point.1.into()))
                .collect();
            res
        })
        .collect::<Vec<Vec<(f64, f64)>>>()
}

pub fn get_chart<'a, 'b>(chart_data: &'a Vec<Vec<(f64, f64)>>, max_x: f32, max_y: f32) -> Chart {
    let mut datasets: Vec<tui::widgets::Dataset> = chart_data
        .into_iter()
        .enumerate()
        .map(|(i, set)| {
            let index: i32 = i.try_into().unwrap();
            Dataset::default()
                // .name("Equation ".to_string() + &index.to_string())
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(COLORS[i]))
                .graph_type(GraphType::Line)
                .data(&set)
        })
        .collect();

    // highlighted_points.into_iter().for_each(|point| {
    //     datasets.push(
    //         Dataset::default()
    //             // .name("Equation ".to_string() + &index.to_string())
    //             .marker(symbols::Marker::Braille)
    //             .style(Style::default().fg(Color::White))
    //             .graph_type(GraphType::Line)
    //             .data(point),
    //     );
    // });

    // datasets.push(
    //     Dataset::default()
    //         // .name("Equation ".to_string() + &index.to_string())
    //         .marker(symbols::Marker::Braille)
    //         .style(Style::default().fg(Color::White))
    //         .graph_type(GraphType::Line)
    //         .data(vec![].as_slice()),
    // );

    Chart::new(datasets)
        .block(Block::default().title("").borders(Borders::NONE))
        .x_axis(
            Axis::default()
                .title("x")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, max_x.into()])
                .labels(vec![]),
        )
        .y_axis(
            Axis::default()
                .title("y")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, max_y.into()])
                .labels(vec![]),
        )
}
