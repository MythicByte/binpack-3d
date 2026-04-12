use serde::{
    Deserialize,
    Serialize,
};

use crate::vector::Vector3;

/// The Bin to the placed into
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bin {
    /// Primary Key
    pub id: i32,
    /// The Position of a Item
    pub position: Vector3<u32>,
    /// Weight
    pub max_weight: u32,
    /// The weight bin has now
    pub weight_currently: u32,
}
impl Bin {
    /// Default Constructor
    pub const fn new(
        id: i32,
        position: Vector3<u32>,
        max_weight: u32,
        weight_currently: u32,
    ) -> Self {
        Self {
            id,
            position,
            max_weight,
            weight_currently,
        }
    }
}
/// Gives for a Bin Space back
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceLeftBin(pub u32);
impl SpaceLeftBin {
    /// Default Constructor
    pub const fn new(input: u32) -> Self {
        Self(input)
    }
}
