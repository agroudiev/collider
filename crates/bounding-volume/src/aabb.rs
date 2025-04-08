use nalgebra::Point3;

/// Axis-Aligned Bounding Box (AABB) structure.
/// An AABB is defined by its minimum and maximum corners in 3D space.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct AABB {
    /// The minimum corner of the AABB.
    pub min: Point3<f32>,
    /// The maximum corner of the AABB.
    pub max: Point3<f32>,
}

impl AABB {
    /// Creates a new AABB with given minimum and maximum corners.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum corner of the AABB.
    /// * `max` - The maximum corner of the AABB.
    pub fn new(min: Point3<f32>, max: Point3<f32>) -> Self {
        AABB { min, max }
    }
}
