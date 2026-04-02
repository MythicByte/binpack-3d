use nalgebra::Vector3;

/// The Bin to the placed into
#[derive(Debug, Clone)]
pub struct Bin {
    /// The Position of a Item
    pub position: Vector3<u32>,
    /// Weight
    pub max_weight: u32,
    /// The weight bin has now
    pub weight_currently: u32,
}
impl Bin {
    /// Default Constructor
    pub const fn new(position: Vector3<u32>, max_weight: u32, weight_currently: u32) -> Self {
        Self {
            position,
            max_weight,
            weight_currently,
        }
    }
}
/// Gives for a Bin Space back
#[derive(Debug, Clone)]
pub struct SpaceLeftBin(pub u32);
impl SpaceLeftBin {
    /// Default Constructor
    pub const fn new(input: u32) -> Self {
        Self(input)
    }
}
