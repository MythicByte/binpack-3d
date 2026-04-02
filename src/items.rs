use nalgebra::Vector3;

/// The Item which should be sorted in the bin
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Item {
    /// The Position of a Item
    pub position: Vector3<u32>,
    /// Weight
    pub weight: u32,
    /// The order of a item should come out
    pub order: u32,
}
impl Item {
    /// Default Constructor
    pub const fn new(position: Vector3<u32>, weight: u32, order: u32) -> Self {
        Self {
            position,
            weight,
            order,
        }
    }
}
/// A item which is in a bin
#[derive(Debug, Clone)]
pub struct ItemsPlaced {
    /// The Position of a Item
    pub position: Vector3<u32>,
    /// Item
    pub item: Item,
}
impl ItemsPlaced {
    /// Default Constructor
    pub const fn new(x: u32, y: u32, z: u32, item: Item) -> Self {
        // Self { x, y, z, item }
        Self {
            position: Vector3::new(x, y, z),
            item,
        }
    }
}
