use pyo3::{pyclass, pymethods};

use crate::Capsule;

#[pyclass(name = "Capsule")]
pub struct PyCapsule {
    pub inner: Capsule,
}

#[pymethods]
impl PyCapsule {
    /// Creates a new capsule with given radius and length along the `z` axis.
    #[new]
    fn new(radius: f32, length: f32) -> Self {
        PyCapsule {
            inner: Capsule::new(radius, length / 2.0),
        }
    }
}
