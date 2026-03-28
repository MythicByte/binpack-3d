use nalgebra::Vector3;

/// The Bin to the placed into
#[derive(Debug)]
pub struct Bin {
    // /// Height
    // pub heigth: f32,
    // /// Width
    // pub width: f32,
    // /// Length
    // pub length: f32,
    /// The Position of a Item
    pub position: Vector3<f32>,
    /// Weight
    pub max_weight: f32,
    /// The weight bin has now
    pub weight_currently: f32,
}
