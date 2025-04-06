/// A capsule shape aligned along the `z`-axis.
///
/// Mathematically, a capsule is the set of points that are at most `radius` units away from the line segment.
/// The line segment is defined by the two endpoints at `(0, 0, -half_length)` and `(0, 0, half_length)`.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Capsule {
    /// The radius of the capsule.
    pub radius: f32,
    /// The half length of the capsule along the `z`-axis.
    pub half_length: f32,
}

impl Capsule {
    /// Creates a new capsule with given radius and half length.
    pub fn new(radius: f32, half_length: f32) -> Self {
        Capsule {
            radius,
            half_length,
        }
    }
}
