/// A 3D mesh shape.
pub struct Mesh {}

/// A Python wrapper for the Mesh type.
#[pyo3::pyclass]
pub struct PyMesh {
    pub inner: Mesh,
}
