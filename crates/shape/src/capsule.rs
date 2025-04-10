use pyo3::{pyclass, pymethods};

use crate::shape::Shape;

/// A capsule shape aligned along the `z`-axis.
///
/// Mathematically, a capsule is the set of points that are at most `radius` units away from the line segment.
/// The line segment is defined by the two endpoints at `(0, 0, -half_length)` and `(0, 0, half_length)`.
#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct Capsule {
    /// The radius of the capsule.
    pub radius: f32,
    /// The half length of the capsule along the `z`-axis.
    pub half_length: f32,
}

impl Capsule {
    /// Creates a new capsule with given radius and half length.
    ///
    /// # Arguments
    ///
    /// * `radius` - The radius of the capsule.
    /// * `half_length` - The half length of the capsule along the `z`-axis.
    pub fn new(radius: f32, half_length: f32) -> Self {
        Capsule {
            radius,
            half_length,
        }
    }
}

impl Shape for Capsule {
    fn is_convex(&self) -> bool {
        true
    }

    fn clone_box(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone())
    }
}

#[pyclass(name = "Capsule")]
pub struct PyCapsule {
    pub inner: Capsule,
}

#[pymethods]
impl PyCapsule {
    /// Creates a new capsule with given radius and half length.
    #[new]
    fn new(radius: f32, half_length: f32) -> Self {
        PyCapsule {
            inner: Capsule::new(radius, half_length),
        }
    }

    /// Returns the radius of the capsule.
    fn radius(&self) -> f32 {
        self.inner.radius
    }

    /// Returns the half length of the capsule.
    fn half_length(&self) -> f32 {
        self.inner.half_length
    }
}
