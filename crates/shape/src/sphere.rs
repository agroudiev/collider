use pyo3::pyclass;

use crate::shape::Shape;

/// A sphere shape.
#[derive(PartialEq, Debug, Copy, Clone)]
#[pyclass(name = "Sphere")]
pub struct Sphere {
    ///  The radius of the sphere.
    pub radius: f32,
}

impl Sphere {
    /// Creates a new sphere with given radius.
    ///
    /// # Arguments
    ///
    /// * `radius` - The radius of the sphere.
    pub fn new(radius: f32) -> Self {
        Sphere { radius }
    }
}

impl Shape for Sphere {
    fn is_convex(&self) -> bool {
        true
    }
}
