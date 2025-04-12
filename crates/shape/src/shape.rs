//! This module defines a trait for geometric shapes and a wrapper type for dynamic dispatch.;

use nalgebra::Vector3;
use pyo3::prelude::*;

/// A wrapper type for the Shape trait to allow dynamic dispatch.
pub type ShapeWrapper = Box<dyn Shape + Send + Sync>;

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
}

#[pyclass]
#[derive(Clone, Copy, Debug)]
pub enum ShapeType {
    Capsule,
    Cone,
    Cuboid,
    Cylinder,
    Sphere,
}

/// A Python wrapper for the ShapeWrapper type.
#[pyo3::pyclass]
pub struct PyShapeWrapper {
    pub inner: ShapeWrapper,
}

#[pyo3::pymethods]
impl PyShapeWrapper {
    #[getter]
    fn get_shape_type(&self) -> PyResult<ShapeType> {
        Ok(self.inner.get_shape_type())
    }

    #[getter]
    fn get_radius(&self) -> PyResult<f32> {
        if let Some(radius) = self.inner.get_radius() {
            Ok(radius)
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(
                "Shape does not have a radius",
            ))
        }
    }

    // #[getter]
    // fn get_half_extents(&self) -> PyResult<Array3<f32>> {
    //     if let Some(half_extents) = self.inner.get_half_extents() {
    //         Ok(half_extents)
    //     } else {
    //         Err(pyo3::exceptions::PyValueError::new_err(
    //             "Shape does not have half extents",
    //         ))
    //     }
    // }

    #[getter]
    fn get_half_length(&self) -> PyResult<f32> {
        if let Some(half_length) = self.inner.get_half_length() {
            Ok(half_length)
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(
                "Shape does not have a half length",
            ))
        }
    }
}
