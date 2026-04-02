use nalgebra::Vector3;

/// The Bin to the placed into
#[derive(Debug)]
pub struct Bin {
    /// The Position of a Item
    pub position: Vector3<u32>,
    /// Weight
    pub max_weight: u32,
    /// The weight bin has now
    pub weight_currently: u32,
}
/// Gives for a Bin Space back
#[derive(Debug)]
pub struct SpaceLeftBin(pub u32);
