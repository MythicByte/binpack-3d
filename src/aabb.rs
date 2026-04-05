use anyhow::anyhow;
use hashbrown::HashMap;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    bin::Bin,
    corners::Corners,
    items::Item,
    vector::Vector6,
};

/// A collision checker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AABBVersion1 {
    grid: HashMap<(u32, u32, u32), Vec<Vector6<u32>>>,
    cell_size: u32,
}
impl Default for AABBVersion1 {
    fn default() -> Self {
        Self {
            grid: Default::default(),
            cell_size: 10,
        }
    }
}
/// Gives a item which can be placed into the grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AABBVersion1CheckedItem(pub Item);

impl AABBVersion1 {
    /// Creates a new AABB Checker
    pub fn new() -> Self {
        Self {
            grid: HashMap::new(),
            cell_size: 10,
        }
    }
    /// Add a new value
    pub fn add(&mut self, item: AABBVersion1CheckedItem, corner: &Corners) -> anyhow::Result<()> {
        let item = item.0;
        let mut position_minimum = corner.position.clone();
        let mut position_maximum = position_minimum.clone() + item.size_cube;
        position_minimum.divide_all(self.cell_size, 1);
        position_maximum.divide_all(self.cell_size, 1);
        let aabb = Vector6::new(
            position_minimum.x.clone(),
            position_minimum.y.clone(),
            position_minimum.z.clone(),
            position_maximum.x.clone(),
            position_maximum.y.clone(),
            position_maximum.z.clone(),
        );
        let end_x = position_maximum.x.saturating_sub(1).max(position_minimum.x);
        let end_y = position_maximum.y.saturating_sub(1).max(position_minimum.y);
        let end_z = position_maximum.z.saturating_sub(1).max(position_minimum.z);
        for x in position_minimum.x..end_x {
            for y in position_minimum.y..end_y {
                for z in position_minimum.z..end_z {
                    self.grid
                        .entry((x, y, z))
                        .or_insert_with(Vec::new)
                        .push(aabb);
                }
            }
        }
        Ok(())
    }
    /// Check if a item does colliot or not
    pub fn check_item(
        &self,
        item: Item,
        corner: &Corners,
    ) -> anyhow::Result<Option<AABBVersion1CheckedItem>> {
        let mut position_minimum = corner.position.clone();
        let mut position_maximum = position_minimum.clone() + item.size_cube.clone();
        position_minimum.divide_all(self.cell_size, 1);
        position_maximum.divide_all(self.cell_size, 1);
        let end_x = position_maximum.x.saturating_sub(1).max(position_minimum.x);
        let end_y = position_maximum.y.saturating_sub(1).max(position_minimum.y);
        let end_z = position_maximum.z.saturating_sub(1).max(position_minimum.z);
        for x in position_minimum.x..end_x {
            for y in position_minimum.y..end_y {
                for z in position_minimum.z..end_z {
                    if let Some(existing) = self.grid.get(&(x, y, z)) {
                        for existing_vector6 in existing {
                            let overlay_x = position_minimum.x < existing_vector6.w
                                && existing_vector6.x < position_maximum.x;
                            let overlay_y = position_minimum.y < existing_vector6.a
                                && existing_vector6.y < position_maximum.y;
                            let overlay_z = position_minimum.z < existing_vector6.b
                                && existing_vector6.z < position_maximum.z;
                            if overlay_x && overlay_y && overlay_z {
                                return Ok(None);
                            }
                        }
                    }
                }
            }
        }
        Ok(Some(AABBVersion1CheckedItem(item)))
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        aabb::AABBVersion1,
        bin::Bin,
        corners::Corners,
        items::Item,
        vector::Vector3,
    };

    #[test]
    fn check_no_collision_and_collision_same_item() {
        let mut aabb = AABBVersion1::new();
        let item = Item::new(Vector3::new(10, 10, 10), 10, 0);
        // check with no item

        let test = aabb
            .check_item(item.clone(), &Corners::new(0, 0, 0))
            .unwrap();
        assert!(test.is_some());
        let test = test.unwrap();
        assert!(aabb.add(test, &Corners::new(0, 0, 0)).is_ok());

        let test = aabb
            .check_item(item.clone(), &Corners::new(0, 0, 0))
            .unwrap();
        assert!(test.is_none());
    }
}
