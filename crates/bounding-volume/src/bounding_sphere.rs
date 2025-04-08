use nalgebra::Point3;

/// A bounding sphere.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct BoundingSphere {
    /// The center of the sphere.
    pub center: Point3<f32>,
    /// The radius of the sphere.
    pub radius: f32,
}

impl BoundingSphere {
    /// Creates a new bounding sphere with given center and radius.
    ///
    /// # Arguments
    ///
    /// * `center` - The center of the sphere.
    /// * `radius` - The radius of the sphere.
    pub fn new(center: Point3<f32>, radius: f32) -> Self {
        BoundingSphere { center, radius }
    }
}
