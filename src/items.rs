/// The Item which should be sorted in the bin
#[derive(Debug)]
pub struct Item {
    x: f32,
    y: f32,
    z: f32,
    /// Height
    pub heigth: f32,
    /// Width
    pub width: f32,
    /// Length
    pub length: f32,
    /// Weight
    pub weight: f32,
    /// The order of a item should come out
    pub order: u32,
}
/// A item which is in a bin
#[derive(Debug)]
pub struct ItemsPlaced {
    x: f32,
    y: f32,
    z: f32,
    item: Item,
}
