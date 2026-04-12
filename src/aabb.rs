use hashbrown::HashMap;
use itertools::iproduct;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
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
    pub fn new(length: usize) -> Self {
        Self {
            grid: HashMap::with_capacity(length),
            cell_size: 8,
        }
    }
    /// Add a new value
    pub fn add(&mut self, item: AABBVersion1CheckedItem, corner: &Corners) -> anyhow::Result<()> {
        let item = item.0;
        let mut position_minimum = corner.position.clone();
        let x_position_minimum = position_minimum.clone();
        let mut position_maximum = position_minimum.clone() + item.size_cube;
        let x_position_maximum = position_maximum.clone();
        position_minimum.divide_all(self.cell_size, 1);
        position_maximum.divide_all(self.cell_size, 1);
        let aabb = Vector6::new(
            x_position_minimum.x.clone(),
            x_position_minimum.y.clone(),
            x_position_minimum.z.clone(),
            x_position_maximum.x.clone(),
            x_position_maximum.y.clone(),
            x_position_maximum.z.clone(),
        );
        let end_x = position_maximum.x;
        let end_y = position_maximum.y;
        let end_z = position_maximum.z;
        for x in position_minimum.x..=end_x {
            for y in position_minimum.y..=end_y {
                for z in position_minimum.z..=end_z {
                    self.grid
                        .entry((x, y, z))
                        .or_insert_with(Vec::new)
                        .push(aabb);
                }
            }
        }
        Ok(())
    }
    /// Version 2 of add
    pub fn add_v2(
        &mut self,
        item: AABBVersion1CheckedItem,
        corner: &Corners,
    ) -> anyhow::Result<()> {
        let item = item.0;
        let mut position_minimum = corner.position.clone();
        let x_position_minimum = position_minimum.clone();
        let mut position_maximum = position_minimum.clone() + item.size_cube;
        let x_position_maximum = position_maximum.clone();
        position_minimum.divide_all(self.cell_size, 1);
        position_maximum.divide_all(self.cell_size, 1);
        let aabb = Vector6::new(
            x_position_minimum.x.clone(),
            x_position_minimum.y.clone(),
            x_position_minimum.z.clone(),
            x_position_maximum.x.clone(),
            x_position_maximum.y.clone(),
            x_position_maximum.z.clone(),
        );
        let end_x = position_maximum.x;
        let end_y = position_maximum.y;
        let end_z = position_maximum.z;
        iproduct!(
            // Check later if = should be here
            position_minimum.x..=end_x,
            position_minimum.y..=end_y,
            position_minimum.z..=end_z
        )
        .for_each(|(x, y, z)| {
            self.grid
                .entry((x, y, z))
                .or_insert_with(Vec::new)
                .push(aabb);
        });
        Ok(())
    }
    /// Check if a item does colliot or not
    pub fn check_item(&self, item: Item, corner: &Corners) -> Option<AABBVersion1CheckedItem> {
        let mut position_minimum = corner.position.clone();
        let mut position_maximum = position_minimum.clone() + item.size_cube.clone();
        let x_position_minimum = position_minimum.clone();
        let x_position_maximum = position_maximum.clone();
        position_minimum.divide_all(self.cell_size, 1);
        position_maximum.divide_all(self.cell_size, 1);
        let end_x = position_maximum.x;
        let end_y = position_maximum.y;
        let end_z = position_maximum.z;
        for x in position_minimum.x..=end_x {
            for y in position_minimum.y..=end_y {
                for z in position_minimum.z..=end_z {
                    if let Some(existing) = self.grid.get(&(x, y, z)) {
                        for existing_vector6 in existing {
                            let overlay_x = x_position_minimum.x < existing_vector6.w
                                && existing_vector6.x < x_position_maximum.x;
                            let overlay_y = x_position_minimum.y < existing_vector6.a
                                && existing_vector6.y < x_position_maximum.y;
                            let overlay_z = x_position_minimum.z < existing_vector6.b
                                && existing_vector6.z < x_position_maximum.z;
                            if overlay_x && overlay_y && overlay_z {
                                return None;
                            }
                        }
                    }
                }
            }
        }
        Some(AABBVersion1CheckedItem(item))
    }

    /// Check v2
    pub fn check_item_v2(&self, item: Item, corner: &Corners) -> Option<AABBVersion1CheckedItem> {
        let mut position_minimum = corner.position.clone();
        let mut position_maximum = position_minimum.clone() + item.size_cube.clone();
        let x_position_minimum = position_minimum.clone();
        let x_position_maximum = position_maximum.clone();
        position_minimum.divide_all(self.cell_size, 1);
        position_maximum.divide_all(self.cell_size, 1);
        let end_x = position_maximum.x;
        let end_y = position_maximum.y;
        let end_z = position_maximum.z;
        let result = iproduct!(
            position_minimum.x..=end_x,
            position_minimum.y..=end_y,
            position_minimum.z..=end_z
        )
        .any(|(x, y, z)| {
            self.grid
                .get(&(x, y, z))
                .map(|existing| {
                    existing.iter().any(|existing_vector6| {
                        let overlay_x = x_position_minimum.x < existing_vector6.w
                            && existing_vector6.x < x_position_maximum.x;
                        let overlay_y = x_position_minimum.y < existing_vector6.a
                            && existing_vector6.y < x_position_maximum.y;
                        let overlay_z = x_position_minimum.z < existing_vector6.b
                            && existing_vector6.z < x_position_maximum.z;
                        overlay_x && overlay_y && overlay_z
                    })
                })
                .unwrap_or_else(|| false)
        });
        // only false no collision
        if result == false {
            return Some(AABBVersion1CheckedItem(item));
        } else {
            None
        }
    }
    /// checks if a point is valid
    pub fn point_is_free(&self, p: &Corners) -> bool {
        let point = p.clone().position;
        let mut cell = p.position;
        cell.divide_all(self.cell_size, 1);

        if let Some(existing) = self.grid.get(&(cell.x, cell.y, cell.z)) {
            for e in existing {
                let inside = point.x >= e.x
                    && point.x < e.w
                    && point.y >= e.y
                    && point.y < e.a
                    && point.z >= e.z
                    && point.z < e.b;
                if inside {
                    return false;
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        aabb::AABBVersion1,
        corners::Corners,
        items::Item,
        vector::Vector3,
    };

    #[test]
    fn check_no_collision_and_collision_same_item() {
        let mut aabb = AABBVersion1::new(10 as usize);
        let item = Item::new(1, Vector3::new(10, 10, 10), 10, 0);
        // check with no item

        let test = aabb.check_item_v2(item.clone(), &Corners::new(0, 0, 0));
        assert!(test.is_some());
        let test = test.unwrap();
        assert!(aabb.add(test, &Corners::new(0, 0, 0)).is_ok());

        let test = aabb.check_item(item.clone(), &Corners::new(0, 0, 0));
        assert!(test.is_none());
    }
}
