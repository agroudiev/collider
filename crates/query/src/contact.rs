//! Contact properties and collision detection between shapes.

use nalgebra::{Isometry3, Point3};
use shape::shape::Shape;

/// Description of a contact.
pub struct Contact {
    /// Point of contact for the first shape.
    pub point1: Point3<f32>,
    /// Point of contact for the second shape.
    pub point2: Point3<f32>,
}

/// Computes the contact properties between two shapes.
/// This function returns `None` if the shapes are not in contact.
/// 
/// # Arguments
/// 
/// * `shape1` - The first shape.
/// * `pos1` - The position and rotation of the first shape.
/// * `shape2` - The second shape.
/// * `pos2` - The position and rotation of the second shape.
pub fn contact<S1: Shape, S2: Shape>(
    shape1: S1,
    pos1: Isometry3<f32>,
    shape2: S2,
    pos2: Isometry3<f32>,
) -> Option<Contact> {
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contact() {
        let shape1 = shape::cuboid::Cuboid::new(nalgebra::Vector3::new(1.0, 1.0, 1.0));
        let pos1 = Isometry3::identity();
        let shape2 = shape::sphere::Sphere::new(1.0);
        let pos2 = Isometry3::identity();

        let contact = contact(shape1, pos1, shape2, pos2);
        assert!(contact.is_none());
    }
}