use nalgebra::Vector3;

/// A item corner
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Corners {
    /// The Position of a Item
    pub position: Vector3<u32>,
}
