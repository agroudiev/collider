//! This crate provides a set of geometric shapes and their properties.
//!
//! A shape is a primitive that represents the properties of a 3D geometric object.
//! Note that a shape does not contain any information about the position
//! or orientation of the object, only its geometric properties.

pub use capsule::Capsule;
pub use cone::Cone;
pub use cuboid::Cuboid;
pub use cylinder::Cylinder;
pub use sphere::Sphere;

pub mod shape;

pub mod capsule;
pub mod cone;
pub mod cuboid;
pub mod cylinder;
pub mod sphere;
