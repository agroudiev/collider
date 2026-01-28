use pyo3::prelude::*;

use collider_rs::{
    mesh::py_mesh::PyMesh,
    shape::{
        py_capsule::PyCapsule, py_cone::PyCone, py_cuboid::PyCuboid, py_cylinder::PyCylinder,
        py_sphere::PySphere,
    },
};

/// A Python module implemented in Rust.
#[pymodule(name = "collider")]
fn collider_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Shapes
    m.add_class::<PyCapsule>()?;
    m.add_class::<PyCone>()?;
    m.add_class::<PyCuboid>()?;
    m.add_class::<PyCylinder>()?;
    m.add_class::<PySphere>()?;

    // Mesh
    m.add_class::<PyMesh>()?;

    Ok(())
}
