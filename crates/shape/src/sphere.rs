#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Sphere {
    ///  The radius of the sphere.
    pub radius: f32,
}

impl Sphere {
    /// Creates a new sphere with given radius.
    pub fn new(radius: f32) -> Self {
        Sphere { radius }
    }
}
