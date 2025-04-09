/// Shape trait for defining geometric shapes.
pub trait Shape {
    /// Returns whether the shape is convex or not.
    fn is_convex(&self) -> bool;
}
