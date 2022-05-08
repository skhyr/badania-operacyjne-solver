use crate::logic::point_satisfy_inequality::point_satisfies_inequalities;
use crate::models::equation::Equation;
use crate::models::point::Point;

pub fn filter_contained_points<'a>(
    equation_system: &Vec<Equation>,
    points: &'a Vec<Point>,
) -> Vec<&'a Point> {
    points
        .into_iter()
        .filter(|point| point_satisfies_inequalities(point, equation_system))
        .collect()
}
