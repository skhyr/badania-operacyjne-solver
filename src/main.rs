use lib::find_intersection::find_intersection;
use lib::linear::{LinearEquality, LinearInequality};
use lib::point::Point;
use lib::render::render;

pub fn find_matrix_intersections(matrix: &Vec<LinearInequality>) -> Vec<Point> {
    let mut points_of_intersection: Vec<Point> = vec![];

    for i1 in 0..matrix.len() {
        for i2 in i1 + 1..matrix.len() {
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
    let score = point.0 * r.0 + point.1 * r.1 - r.2;
    score <= 0.0
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
    points
        .into_iter()
        .filter(|point| point_satisfies_inequalities(point, matrix))
        .collect()
}

pub fn find_optimum(matrix: &Vec<LinearInequality>, score_fn: &LinearEquality) -> (Point, f32) {
    let points_of_intersection = find_matrix_intersections(&matrix);

    let vertices = filter_contained_points(&matrix, &points_of_intersection);
    let mut scores: Vec<(&Point, f32)> = vertices
        .into_iter()
        .map(|point| (point, point.0 * score_fn.0 + score_fn.1 * point.1))
        .collect();

    scores.sort_by(|a, b| {
        let v1 = a.1;
        let v2 = b.1;
        v2.partial_cmp(&v1).unwrap()
    });
    // println!("#{:?}", scores);
    let best_point = scores[0].0;
    let best_score = scores[0].1;
    (Point(best_point.0, best_point.1), best_score)
}

fn main() {
    render();
    let matrix = vec![
        LinearInequality(12.0, 4.0, 480.0, false),
        LinearInequality(8.0, 8.0, 640.0, false),
        LinearInequality(1.0, -1.0, 0.0, false),
        LinearInequality(-1.0, 0.0, 0.0, false),
        LinearInequality(0.0, -1.0, 0.0, false),
    ];
    let score_fn = LinearEquality(50.0, 10.0, 0.0);

    let res = find_optimum(&matrix, &score_fn);
    println!("#{:?}", res);
}
