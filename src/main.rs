use lib::find_intersection::find_intersection;
use lib::linear::LinearInequality;
use lib::point::Point;

pub fn find_matrix_intersections(matrix: &Vec<LinearInequality>) -> Vec<Point> {
    let mut points_of_intersection: Vec<Point> = vec![];

    for i1 in 0..6 {
        for i2 in i1 + 1..6 {
            let r1 = &matrix[i1];
            let r2 = &matrix[i2];

            let point = find_intersection(
                LinearInequality::to_equality(&r1),
                LinearInequality::to_equality(&r2),
            );

            points_of_intersection.push(point)
        }
    }

    points_of_intersection
}

pub fn point_satisfies_inequality(point: &Point, r: &LinearInequality) -> bool {
    point.0 * r.0 + point.1 * r.1 - r.2 <= 0.0
}

pub fn point_satisfies_inequalities(point: &Point, rs: &Vec<LinearInequality>) -> bool {
    let res: Vec<bool> = rs
        .into_iter()
        .map(|r| point_satisfies_inequality(point, r))
        .collect();

    let res2: Vec<bool> = res.into_iter().filter(|e| *e != true).collect();
    res2.is_empty()
}

pub fn filter_contained_points<'a>(
    matrix: &Vec<LinearInequality>,
    points: &'a Vec<Point>,
) -> Vec<&'a Point> {
    let vertices: Vec<&Point> = points
        .into_iter()
        .filter(|point| point_satisfies_inequalities(point, matrix))
        .collect();
    vertices
}

fn main() {
    let matrix = vec![
        LinearInequality(3.0, 1.0, 33.0, false),
        LinearInequality(1.0, 1.0, 13.0, false),
        LinearInequality(5.0, 8.0, 80.0, false),
        LinearInequality(0.0, 1.0, 7.0, false),
        LinearInequality(-1.0, 0.0, 0.0, false),
        LinearInequality(0.0, -1.0, 0.0, false),
    ];

    let points_of_intersection = find_matrix_intersections(&matrix);

    let vertices = filter_contained_points(&matrix, &points_of_intersection);
    println!("#{:?}", vertices);
}
