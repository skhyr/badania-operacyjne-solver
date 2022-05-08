use crate::logic::filter_contained_points::filter_contained_points;
use crate::logic::find_intersection::find_system_intersections;
use crate::models::equation::Equation;
use crate::models::point::Point;

pub fn find_optimum(matrix: &Vec<Equation>, score_fn: &Equation) -> (Point, f32) {
    let points_of_intersection = find_system_intersections(&matrix);

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
    let best_point = scores[0].0;
    let best_score = scores[0].1;
    (Point(best_point.0, best_point.1), best_score)
}
