use nalgebra::Vector3;
use pyo3::pyclass;

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
