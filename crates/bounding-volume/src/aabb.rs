use nalgebra::Point3;

use crate::BoundingVolume;

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

impl BoundingVolume for AABB {
    fn center(&self) -> Point3<f32> {
        nalgebra::center(&self.min, &self.max)
    }

    fn contains_point(&self, point: &nalgebra::Point3<f32>) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    fn contains_volume(&self, other: &Self) -> bool {
        self.min.x <= other.min.x
            && self.max.x >= other.max.x
            && self.min.y <= other.min.y
            && self.max.y >= other.max.y
            && self.min.z <= other.min.z
            && self.max.z >= other.max.z
    }

    fn scale(&mut self, scale: f32) {
        let center = self.center();
        let half_size = (self.max - self.min) * 0.5 * scale;
        self.min = center - half_size;
        self.max = center + half_size;
    }

    fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }
}
