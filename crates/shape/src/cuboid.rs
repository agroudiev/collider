pub struct Cuboid {
    /// The half extents of the cuboid.
    pub half_extents: [f32; 3],
}

impl Cuboid {
    /// Creates a new cuboid with given half extents.
    pub fn new(half_extents: [f32; 3]) -> Self {
        Cuboid { half_extents }
    }
}
