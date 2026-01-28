use pyo3::{pyclass, pymethods};

use crate::Cone;

#[pyclass(name = "Cone")]
pub struct PyCone {
    pub inner: Cone,
}

#[pymethods]
impl PyCone {
    /// Creates a new cone with given radius and half height.
    ///
    /// # Arguments
    ///
    /// * `radius` - The radius of the cone.
    /// * `ength` - The length of the cone along the `z`-axis.
    #[new]
    fn new(radius: f32, length: f32) -> Self {
        PyCone {
            inner: Cone::new(radius, length / 2.0),
        }
    }

    /// Returns the radius of the cone.
    fn radius(&self) -> f32 {
        self.inner.radius
    }

    /// Returns the half length of the cone.
    fn half_length(&self) -> f32 {
        self.inner.half_length
    }
}
