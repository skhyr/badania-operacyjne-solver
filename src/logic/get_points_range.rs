use crate::models::equation::Equation;
use crate::models::point::Point;
pub fn get_points_range(equation: &Equation) -> Vec<Point> {
    vec![
        equation.point(0.0),
        equation.point_of_value(1.0),
        equation.point(100.0),
    ]
}
