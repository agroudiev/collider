use nalgebra::Vector3;
use pyo3::{pyclass, pymethods};

use crate::shape::Shape;

/// A cuboid shape.
///
/// The cuboid is zero-centered and defined by its half extents along the `x`, `y`, and `z` axes.
#[derive(PartialEq, Debug, Clone, Copy)]
#[pyclass(name = "Cuboid")]
pub struct Cuboid {
    /// The half extents of the cuboid.
    pub half_extents: Vector3<f32>,
}

impl Cuboid {
    /// Creates a new cuboid with given half extents.
    ///
    /// # Arguments
    ///
    /// * `half_extents` - The half extents of the cuboid along the `x`, `y`, and `z` axes.
    pub fn new(half_extents: Vector3<f32>) -> Self {
        Cuboid { half_extents }
    }
}

impl Shape for Cuboid {
    fn is_convex(&self) -> bool {
        true
    }
}


#[pyclass(name = "Cuboid")]
pub struct PyCuboid {
    inner: Cuboid,
}

#[pymethods]
impl PyCuboid {
    /// Creates a new cuboid with given half extents.
    ///
    /// # Arguments
    ///
    /// * `half_extents` - The half extents of the cuboid along the `x`, `y`, and `z` axes.
    #[new]
    fn new(half_extents: Vec<f32>) -> Self {
        PyCuboid {
            inner: Cuboid::new(Vector3::new(half_extents[0], half_extents[1], half_extents[2])),
        }
    }

    /// Returns the half extents of the cuboid.
    fn half_extents(&self) -> Vec<f32> {
        vec![self.inner.half_extents.x, self.inner.half_extents.y, self.inner.half_extents.z]
    }
}