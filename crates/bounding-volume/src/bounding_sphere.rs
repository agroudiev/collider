use nalgebra::Point3;

use crate::BoundingVolume;

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

impl BoundingVolume for BoundingSphere {
    fn center(&self) -> Point3<f32> {
        self.center
    }

    fn contains_point(&self, point: &Point3<f32>) -> bool {
        nalgebra::distance_squared(&self.center, point) <= self.radius * self.radius
    }

    fn contains_volume(&self, other: &Self) -> bool {
        nalgebra::distance_squared(&self.center, &other.center) + other.radius * other.radius
            <= self.radius * self.radius
    }

    fn scale(&mut self, scale: f32) {
        self.radius *= scale;
    }

    fn intersects(&self, other: &Self) -> bool {
        nalgebra::distance_squared(&self.center, &other.center)
            <= (self.radius + other.radius) * (self.radius + other.radius)
    }
}
