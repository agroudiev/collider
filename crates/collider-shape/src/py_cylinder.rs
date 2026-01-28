use pyo3::{pyclass, pymethods};

use crate::Cylinder;

#[pyclass(name = "Cylinder")]
pub struct PyCylinder {
    pub inner: Cylinder,
}

#[pymethods]
impl PyCylinder {
    /// Creates a new cylinder with given radius and half height.
    ///
    /// # Arguments
    ///
    /// * `radius` - The radius of the cylinder.
    /// * `length` - The length of the cylinder along the `z`-axis.
    #[new]
    fn new(radius: f32, length: f32) -> Self {
        PyCylinder {
            inner: Cylinder::new(radius, length / 2.0),
        }
    }

    /// Returns the radius of the cylinder.
    fn radius(&self) -> f32 {
        self.inner.radius
    }

    /// Returns the half length of the cylinder.
    fn half_length(&self) -> f32 {
        self.inner.half_length
    }
}
