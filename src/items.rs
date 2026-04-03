use nalgebra::Vector3;

use crate::corners::Corners;

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
    /// How to rotate the item in all directions
    pub fn rotation(&self) -> (Corners, Corners, Corners, Corners, Corners, Corners) {
        let (x, y, z) = (self.position.x, self.position.y, self.position.z);
        let create_corner = |x: Vector3<u32>| Corners::from(x);
        let first_rotation = create_corner(Vector3::new(y, x, z));
        let second_rotation = create_corner(Vector3::new(x, z, y));
        let third_rotation = create_corner(Vector3::new(x, z, y));
        let four_rotation = create_corner(Vector3::new(z, x, y));
        let five_rotation = create_corner(Vector3::new(y, z, x));
        let sixs_rotation = create_corner(Vector3::new(z, y, x));
        (
            first_rotation,
            second_rotation,
            third_rotation,
            four_rotation,
            five_rotation,
            sixs_rotation,
        )
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
    pub const fn new(position: Vector3<u32>, item: Item) -> Self {
        // Self { x, y, z, item }
        Self {
            position: position,
            item,
        }
    }
}
