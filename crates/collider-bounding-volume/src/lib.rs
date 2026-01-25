//! # **`collider-bounding-volume`**: bounding volumes for collision detection
//!
//! `collider-bounding-volume` is a sub-crate of the `collider` library, providing
//! definitions and implementations of bounding volumes used in collision detection.
//!
//! ## Overview
//! A bounding volume is a simple geometric shape that completely contains a more complex shape.
//! Bounding volumes are used to optimize collision detection by quickly ruling out
//! potential collisions before performing more detailed checks.
//!
//! Conversely to shapes, bounding volumes do not represent the actual geometry of an object,
//! but rather a simplified approximation that is easier to compute with. Volumes also
//! store information about their position and orientation in space, which shapes do not.

pub mod aabb;
pub mod bounding_sphere;
pub mod bounding_volume;
