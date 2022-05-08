use crate::models::equation::Equation;
use crate::models::point::Point;
pub fn get_points_range(equation: &Equation, max_x: f32, max_y: f32) -> Vec<Point> {
    let mut res = vec![];
    let max: i32 = max_x.floor() as i32;
    for i in 0..max {
        res.push(equation.point(i as f32));
    }
    if equation.1 == 0.0 {
        res.push(Point(0.0, 0.0));
        res.push(Point(0.0, max_y))
    };
    res
}
