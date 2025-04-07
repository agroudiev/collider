use pyo3::prelude::*;

use collider::shape::{Capsule, Cone, Cuboid, Cylinder, Sphere};

/// A Python module implemented in Rust.
#[pymodule(name = "collider")]
fn collider_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Shapes
    m.add_class::<Capsule>()?;
    m.add_class::<Cone>()?;
    m.add_class::<Cuboid>()?;
    m.add_class::<Cylinder>()?;
    m.add_class::<Sphere>()?;

    Ok(())
}
