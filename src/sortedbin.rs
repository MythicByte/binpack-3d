use crate::{
    bin::Bin,
    items::ItemsPlaced,
};

/// A Bin which has the Items in It
#[derive(Debug)]
pub struct SortedBin {
    /// Bin
    pub bin: Bin,
    /// Items
    pub items: Vec<ItemsPlaced>,
}
impl SortedBin {
    /// Creates basic items
    pub fn new(bin: Bin, items: Vec<ItemsPlaced>) -> Self {
        Self { bin, items }
    }
}
