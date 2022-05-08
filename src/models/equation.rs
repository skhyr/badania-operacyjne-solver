use crate::models::point::Point;

#[derive(Debug)]
pub struct Equation(pub f32, pub f32, pub f32);

impl Equation {
    pub fn value(&self, x: f32) -> f32 {
        (self.2 - self.0 * x) / self.1
    }

    pub fn value_position(&self, y: f32) -> f32 {
        (self.2 / self.1 * y) / self.0
    }

    pub fn point_of_value(&self, y: f32) -> Point {
        Point(self.value_position(y), y)
    }

    pub fn point(&self, x: f32) -> Point {
        Point(x, self.value(x))
    }
}
