/// A 3D mesh shape.
pub struct Mesh {
    /// The path to the mesh file.
    pub path: String,
}

impl Mesh {
    /// Creates a new Mesh from the given file path.
    pub fn new(path: String) -> Self {
        Mesh { path }
    }
}

/// A Python wrapper for the Mesh type.
#[pyo3::pyclass]
pub struct PyMesh {
    pub inner: Mesh,
}

#[pyo3::pymethods]
impl PyMesh {
    #[new]
    fn new(path: String) -> Self {
        PyMesh { inner: Mesh { path } }
    }

    #[getter]
    fn get_mesh_path(&self) -> String {
        self.inner.path.clone()
    }
}