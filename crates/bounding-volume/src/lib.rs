pub mod aabb;
pub mod bounding_sphere;

trait BoundingVolume {
    /// Returns the center of the bounding volume.
    ///
    /// # Returns
    ///
    /// The center of the bounding volume as a `Point3<f32>`.
    fn center(&self) -> nalgebra::Point3<f32>;

    /// Check if the bouding volume contains a point.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to check for containment.
    fn contains_point(&self, point: &nalgebra::Point3<f32>) -> bool;

    /// Check if the bouding volume contains another bounding volume of the same type.
    ///
    /// # Arguments
    ///
    /// * `other` - The other bounding volume to check for containment.
    fn contains_volume(&self, other: &Self) -> bool;

    /// Scales the bounding volume by a given factor.
    /// 
    /// # Arguments
    /// 
    /// * `scale` - The scaling factor.
    fn scale(&mut self, scale: f32);

    /// Checks if this bounding volume intersects with another bounding volume of the same type.
    ///
    /// # Arguments
    ///
    /// * `other` - The other bounding volume to check for intersection.
    fn intersects(&self, other: &Self) -> bool;
}
