//! This module defines a trait for geometric shapes and a wrapper type for dynamic dispatch.

use nalgebra::Vector3;
use std::fmt::Debug;

/// A type alias for an object implementing the Shape trait.
/// This is separated from ShapeWrapper to allow for implementing the Debug trait.
pub type InnerShapeWrapper = dyn Shape + Send + Sync;

impl Debug for &InnerShapeWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InnerShapeWrapper")
            .field("shape_type", &self.get_shape_type())
            .field("radius", &self.get_radius())
            .field("half_extents", &self.get_half_extents())
            .field("half_length", &self.get_half_length())
            .finish()
    }
}

impl PartialEq for &InnerShapeWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.get_shape_type() == other.get_shape_type()
            && self.get_radius() == other.get_radius()
            && self.get_half_extents() == other.get_half_extents()
            && self.get_half_length() == other.get_half_length()
    }
}

/// A wrapper type for the Shape trait to allow dynamic dispatch.
pub type ShapeWrapper = Box<InnerShapeWrapper>;

impl PartialEq for ShapeWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.get_shape_type() == other.get_shape_type()
            && self.get_radius() == other.get_radius()
            && self.get_half_extents() == other.get_half_extents()
            && self.get_half_length() == other.get_half_length()
    }
}

/// Shape trait for defining geometric shapes.
pub trait Shape {
    /// Returns whether the shape is convex or not.
    fn is_convex(&self) -> bool;

    /// Clones the shape and returns a boxed version of it.
    fn clone_box(&self) -> ShapeWrapper;

    /// Returns the shape type.
    fn get_shape_type(&self) -> ShapeType;

    /// Returns the radius of the shape.
    fn get_radius(&self) -> Option<f32> {
        None
    }

    /// Returns the half extents of the shape.
    fn get_half_extents(&self) -> Option<Vector3<f32>> {
        None
    }

    /// Returns the half length of the shape.
    fn get_half_length(&self) -> Option<f32> {
        None
    }

    /// Returns the mesh path of the shape.
    fn get_mesh_path(&self) -> Option<String> {
        None
    }
}

#[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeType {
    Capsule,
    Cone,
    Cuboid,
    Cylinder,
    Sphere,
    Mesh,
}
