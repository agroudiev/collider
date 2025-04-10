//! This module defines a trait for geometric shapes and a wrapper type for dynamic dispatch.;

/// Shape trait for defining geometric shapes.
pub trait Shape {
    /// Returns whether the shape is convex or not.
    fn is_convex(&self) -> bool;

    /// Clones the shape and returns a boxed version of it.
    fn clone_box(&self) -> Box<dyn Shape + Send + Sync>;
}

/// A wrapper type for the Shape trait to allow dynamic dispatch.
pub type ShapeWrapper = Box<dyn Shape + Send + Sync>;

/// A Python wrapper for the ShapeWrapper type.
#[pyo3::pyclass]
pub struct PyShapeWrapper {
    pub inner: ShapeWrapper,
}
