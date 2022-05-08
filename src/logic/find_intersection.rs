use crate::models::equation::Equation;
use crate::models::point::Point;

pub fn find_intersection(r1: &Equation, r2: &Equation) -> Point {
    let a = r1.0;
    let b = r1.1;
    let e = r1.2;
    let c = r2.0;
    let d = r2.1;
    let f = r2.2;

    let x = (e * d - b * f) / (a * d - b * c);
    let y = (a * f - e * c) / (a * d - b * c);
    Point(x.into(), y.into())
}

pub fn find_system_intersections(equation_system: &Vec<Equation>) -> Vec<Point> {
    let mut points_of_intersection: Vec<Point> = vec![];

    for i1 in 0..equation_system.len() {
        for i2 in i1 + 1..equation_system.len() {
            let r1 = &equation_system[i1];
            let r2 = &equation_system[i2];
            let point = find_intersection(r1, r2);
            points_of_intersection.push(point)
        }
    }

    points_of_intersection
}
