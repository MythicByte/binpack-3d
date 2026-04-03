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
impl From<Vector3<u32>> for Corners {
    fn from(value: Vector3<u32>) -> Self {
        Self { position: value }
    }
}
