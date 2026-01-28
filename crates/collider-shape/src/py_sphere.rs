use pyo3::{pyclass, pymethods};

use crate::Sphere;

#[pyclass(name = "Sphere")]
pub struct PySphere {
    pub inner: Sphere,
}

#[pymethods]
impl PySphere {
    /// Creates a new sphere with given radius.
    ///
    /// # Arguments
    ///
    /// * `radius` - The radius of the sphere.
    #[new]
    fn new(radius: f32) -> Self {
        PySphere {
            inner: Sphere::new(radius),
        }
    }
}
