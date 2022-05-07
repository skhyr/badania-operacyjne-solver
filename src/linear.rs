#[derive(Debug)]
pub struct LinearInequality(pub f32, pub f32, pub f32, pub bool);

#[derive(Debug)]
pub struct LinearEquality(pub f32, pub f32, pub f32);

impl LinearInequality {
    pub fn to_equality(r1: &LinearInequality) -> LinearEquality {
        LinearEquality(r1.0, r1.1, r1.2)
    }
}
