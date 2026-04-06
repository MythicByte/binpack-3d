use serde::{
    Deserialize,
    Serialize,
};

use crate::vector::Vector3;

/// A item corner
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Corners {
    /// The Position of a Item
    pub position: Vector3<u32>,
}

impl Corners {
    /// Creates a new corner
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        // Self { x, y, z }
        Self {
            position: Vector3::new(x, y, z),
        }
    }
}
impl From<Vector3<u32>> for Corners {
    fn from(value: Vector3<u32>) -> Self {
        Self { position: value }
    }
}
