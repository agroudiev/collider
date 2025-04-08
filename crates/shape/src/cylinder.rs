use pyo3::pyclass;

use crate::shape::Shape;

/// A cylinder shape aligned along the `z`-axis.
///
/// The base of the cylinder is at `(0, 0, -half_length)` and the top is at `(0, 0, half_length)`.
#[derive(PartialEq, Debug, Copy, Clone)]
#[pyclass(name = "Cylinder")]
pub struct Cylinder {
    /// The radius of the cylinder.
    pub radius: f32,
    /// The half length of the cylinder along the `z`-axis.
    pub half_length: f32,
}

impl Cylinder {
    /// Creates a new cylinder with given radius and half height.
    ///
    /// # Arguments
    ///
    /// * `radius` - The radius of the cylinder.
    /// * `half_length` - The half length of the cylinder along the `z`-axis.
    pub fn new(radius: f32, half_length: f32) -> Self {
        Cylinder {
            radius,
            half_length,
        }
    }
}

impl Shape for Cylinder {
    fn is_convex(&self) -> bool {
        true
    }
}
