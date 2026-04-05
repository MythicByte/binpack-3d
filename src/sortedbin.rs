use crate::{
    bin::Bin,
    items::{
        Item,
        ItemsPlaced,
    },
};

/// A Bin which has the Items in It
#[derive(Debug)]
pub struct SortedBin {
    /// Bin
    pub bin: Bin,
    /// Items
    pub items: Vec<ItemsPlaced>,
    /// removed items
    pub removed_items: Vec<Item>,
}
impl SortedBin {
    /// Creates basic items
    pub fn new(bin: Bin, items: Vec<ItemsPlaced>) -> Self {
        Self {
            bin,
            items,
            removed_items: Vec::new(),
        }
    }
}
