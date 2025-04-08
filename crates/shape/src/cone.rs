use pyo3::pyclass;

/// A cone shape aligned along the `z`-axis.
///
/// The base of the cone is at `(0, 0, -half_length)` and the tip is at `(0, 0, half_length)`.
#[derive(PartialEq, Debug, Copy, Clone)]
#[pyclass(name = "Cone")]
pub struct Cone {
    /// The radius of the cone.
    pub radius: f32,
    /// The half length of the cone along the `z`-axis.
    pub half_length: f32,
}
impl Cone {
    /// Creates a new cone with given radius and half height.
    ///
    /// # Arguments
    ///
    /// * `radius` - The radius of the cone.
    /// * `half_length` - The half length of the cone along the `z`-axis.
    pub fn new(radius: f32, half_length: f32) -> Self {
        Cone {
            radius,
            half_length,
        }
    }
}
