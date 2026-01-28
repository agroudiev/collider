use crate::Mesh;
use pyo3::prelude::*;

/// A Python wrapper for the Mesh type.
#[pyclass]
pub struct PyMesh {
    pub inner: Mesh,
}

#[pymethods]
impl PyMesh {
    #[new]
    fn new(path: String) -> Self {
        PyMesh {
            inner: Mesh { path },
        }
    }

    #[getter]
    fn get_mesh_path(&self) -> String {
        self.inner.path.clone()
    }
}
