use crate::logic::find_intersection::find_system_intersections;
use crate::logic::find_optimum::find_scores;
use crate::models::equation::Equation;
use tui::{
    layout::Corner,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
};

pub fn get_result(equations: &Vec<Equation>, score_fn: &Equation) -> List<'static> {
    let points = find_system_intersections(equations);
    let result = find_scores(equations, points, score_fn);

    let list: Vec<ListItem> = result
        .into_iter()
        .map(|(point, score)| {
            ListItem::new(Span::styled(
                score.to_string()
                    + " "
                    + "("
                    + &point.0.to_string()
                    + ";"
                    + &point.1.to_string()
                    + ")",
                Style::default(),
            ))
        })
        .collect();

    List::new(list)
        .block(Block::default().borders(Borders::ALL).title("Equations"))
        .start_corner(Corner::TopLeft)
}
