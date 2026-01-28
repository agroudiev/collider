use nalgebra::Vector3;
use pyo3::prelude::*;

use crate::Cuboid;

#[pyclass(name = "Cuboid")]
pub struct PyCuboid {
    pub inner: Cuboid,
}

#[pymethods]
impl PyCuboid {
    /// Creates a new cuboid with given half extents.
    ///
    /// # Arguments
    ///
    /// * `sides` - The length of the sides of the cuboid along the `x`, `y`, and `z` axes.
    #[new]
    fn new(sides: Vec<f32>) -> Self {
        PyCuboid {
            inner: Cuboid::new(Vector3::new(sides[0] / 2.0, sides[1] / 2.0, sides[2] / 2.0)),
        }
    }
}
