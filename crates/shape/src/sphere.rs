use pyo3::{pyclass, pymethods};

use crate::shape::Shape;

/// A sphere shape.
#[derive(PartialEq, Debug, Copy, Clone)]
#[pyclass(name = "Sphere")]
pub struct Sphere {
    ///  The radius of the sphere.
    pub radius: f32,
}

impl Sphere {
    /// Creates a new sphere with given radius.
    ///
    /// # Arguments
    ///
    /// * `radius` - The radius of the sphere.
    pub fn new(radius: f32) -> Self {
        Sphere { radius }
    }
}

impl Shape for Sphere {
    fn is_convex(&self) -> bool {
        true
    }

    fn clone_box(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(*self)
    }

    fn get_shape_type(&self) -> crate::shape::ShapeType {
        crate::shape::ShapeType::Sphere
    }

    fn get_radius(&self) -> Option<f32> {
        Some(self.radius)
    }
}

#[pyclass(name = "Sphere")]
pub struct PySphere {
    pub inner: Sphere,
}

#[pymethods]
impl PySphere {
    /// Creates a new sphere with given radius.
    ///
    /// # Arguments
    ///
    /// * `radius` - The radius of the sphere.
    #[new]
    fn new(radius: f32) -> Self {
        PySphere {
            inner: Sphere::new(radius),
        }
    }
}
