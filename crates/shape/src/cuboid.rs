use pyo3::pyclass;

/// A cuboid shape.
///
/// The cuboid is zero-centered and defined by its half extents along the `x`, `y`, and `z` axes.
#[derive(PartialEq, Debug, Clone, Copy)]
#[pyclass(name = "Cuboid")]
pub struct Cuboid {
    /// The half extents of the cuboid.
    pub half_extents: [f32; 3],
}

impl Cuboid {
    /// Creates a new cuboid with given half extents.
    pub fn new(half_extents: [f32; 3]) -> Self {
        Cuboid { half_extents }
    }
}
