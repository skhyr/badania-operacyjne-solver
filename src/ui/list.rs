use crate::models::equation::Equation;
use crate::ui::colors::COLORS;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

pub fn get_list(equations: &Vec<Equation>) -> List<'static> {
    let events: Vec<ListItem> = equations
        .into_iter()
        .enumerate()
        .map(|(i, equation)| {
            ListItem::new(Span::styled(
                equation.to_string(),
                Style::default().fg(COLORS[i]),
            ))
        })
        .collect();

    List::new(events)
        .block(Block::default().borders(Borders::ALL).title("Equations"))
        .start_corner(Corner::TopLeft)
}
