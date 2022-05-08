use crate::linear::LinearEquality;
use crate::point::Point;

pub fn find_intersection(r1: LinearEquality, r2: LinearEquality) -> Point {
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
