use crate::logic::filter_contained_points::filter_contained_points;
use crate::logic::find_intersection::find_system_intersections;
use crate::models::equation::Equation;
use crate::models::point::Point;

pub fn find_scores(
    matrix: &Vec<Equation>,
    points: Vec<Point>,
    score_fn: &Equation,
) -> Vec<(Point, f32)> {
    let vertices = filter_contained_points(&matrix, &points);
    let mut scores: Vec<(Point, f32)> = vec![];

    vertices.into_iter().for_each(|point| {
        let val = (*point, point.0 * score_fn.0 + score_fn.1 * point.1);
        scores.push(val);
    });

    scores.sort_by(|a, b| {
        let v1 = a.1;
        let v2 = b.1;
        v2.partial_cmp(&v1).unwrap()
    });
    scores
}

pub fn find_optimum(matrix: &Vec<Equation>, score_fn: &Equation) -> (Point, f32) {
    let points = find_system_intersections(matrix);
    let scores = find_scores(matrix, points, score_fn);
    let best_point = scores[0].0;
    let best_score = scores[0].1;
    (Point(best_point.0, best_point.1), best_score)
}
