/// The Item which should be sorted in the bin
#[derive(Debug)]
pub struct Item {
    /// x
    pub x: f32,
    /// y
    pub y: f32,
    /// z
    pub z: f32,
    /// Weight
    pub weight: f32,
    /// The order of a item should come out
    pub order: u32,
}
/// A item which is in a bin
#[derive(Debug)]
pub struct ItemsPlaced {
    /// x
    pub x: f32,
    /// y
    pub y: f32,
    /// z
    pub z: f32,
    /// Item
    pub item: Item,
}
impl ItemsPlaced {
    /// Default Constructor
    pub fn new(x: f32, y: f32, z: f32, item: Item) -> Self {
        Self { x, y, z, item }
    }
}
