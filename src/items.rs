use serde::{
    Deserialize,
    Serialize,
};

use crate::vector::Vector3;

/// The Item which should be sorted in the bin
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Item {
    /// The size of a Item
    pub size_cube: Vector3<u32>,
    /// Weight
    pub weight: u32,
    /// The order of a item should come out
    pub order: u32,
}
impl Item {
    /// Default Constructor
    pub const fn new(position: Vector3<u32>, weight: u32, order: u32) -> Self {
        Self {
            size_cube: position,
            weight,
            order,
        }
    }
    /// How to rotate the item in all directions
    pub fn rotation(&self) -> Vec<Vector3<u32>> {
        let (x, y, z) = (self.size_cube.x, self.size_cube.y, self.size_cube.z);
        let first_rotation = Vector3::new(y, x, z);
        let second_rotation = Vector3::new(x, z, y);
        let four_rotation = Vector3::new(z, x, y);
        let five_rotation = Vector3::new(y, z, x);
        let sixs_rotation = Vector3::new(z, y, x);
        let normal_rotation = Vector3::new(x, y, z);
        vec![
            normal_rotation,
            first_rotation,
            second_rotation,
            four_rotation,
            five_rotation,
            sixs_rotation,
        ]
    }
    /// ratation v2
    pub fn rotation_v2(&self) -> Vec<Item> {
        let (x, y, z) = (self.size_cube.x, self.size_cube.y, self.size_cube.z);
        let first_rotation = Item::new(Vector3::new(y, x, z), self.weight, self.order);
        let second_rotation = Item::new(Vector3::new(x, z, y), self.weight, self.order);
        let four_rotation = Item::new(Vector3::new(z, x, y), self.weight, self.order);
        let five_rotation = Item::new(Vector3::new(y, z, x), self.weight, self.order);
        let sixs_rotation = Item::new(Vector3::new(z, y, x), self.weight, self.order);
        let normal_rotation = Item::new(Vector3::new(x, y, z), self.weight, self.order);
        vec![
            normal_rotation,
            first_rotation,
            second_rotation,
            four_rotation,
            five_rotation,
            sixs_rotation,
        ]
    }
}
/// A item which is in a bin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemsPlaced {
    /// The Position of a Item
    pub position: Vector3<u32>,
    /// Item
    pub item: Item,
}
impl ItemsPlaced {
    /// Default Constructor
    pub const fn new(position: Vector3<u32>, item: Item) -> Self {
        // Self { x, y, z, item }
        Self { position, item }
    }
}
