use crate::models::equation::Equation;
use crate::models::point::Point;

pub fn point_satisfies_inequality(point: &Point, r: &Equation) -> bool {
    let score = point.0 * r.0 + point.1 * r.1 - r.2;
    score <= 0.0
}

pub fn point_satisfies_inequalities(point: &Point, rs: &Vec<Equation>) -> bool {
    let res: Vec<bool> = rs
        .into_iter()
        .map(|r| point_satisfies_inequality(point, r))
        .collect();
    let res2: Vec<bool> = res.into_iter().filter(|e| *e != true).collect();
    res2.is_empty()
}
