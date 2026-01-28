use numpy::{ToPyArray, ndarray::Array1};
use pyo3::prelude::*;

use crate::{ShapeType, ShapeWrapper};

/// A Python wrapper for the ShapeWrapper type.
#[pyo3::pyclass(name = "Shape")]
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

    #[getter]
    fn get_half_extents(&self, py: Python) -> PyResult<Py<PyAny>> {
        if let Some(half_extents) = self.inner.get_half_extents() {
            Ok(Array1::from_shape_vec(3, half_extents.as_slice().to_vec())
                .unwrap()
                .to_pyarray(py)
                .into_any()
                .unbind())
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(
                "Shape does not have half extents",
            ))
        }
    }

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

    #[getter]
    fn get_mesh_path(&self) -> PyResult<String> {
        if let Some(path) = self.inner.get_mesh_path() {
            Ok(path)
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(
                "Shape is not a mesh and does not have a path",
            ))
        }
    }
}
