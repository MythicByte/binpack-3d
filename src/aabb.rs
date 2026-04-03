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
    bin: Bin,
}
/// Gives a item which can be placed into the grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AABBVersion1CheckedItem(pub Item);

impl AABBVersion1 {
    /// Creates a new AABB Checker
    pub fn new(bin: Bin) -> Self {
        Self {
            grid: HashMap::new(),
            cell_size: 10,
            bin,
        }
    }
    /// Add a new value
    pub fn add(&mut self, item: AABBVersion1CheckedItem, corner: &Corners) -> anyhow::Result<()> {
        let item = item.0;
        let mut position_minimum = item.size_cube.clone() + corner.position.clone();
        let mut position_maximum = position_minimum.clone() + item.size_cube;
        position_minimum.divide_all(self.cell_size);
        position_maximum.divide_all(self.cell_size);
        let aabb = Vector6::new(
            position_minimum.x.clone(),
            position_minimum.y.clone(),
            position_minimum.z.clone(),
            position_maximum.x.clone(),
            position_maximum.y.clone(),
            position_maximum.z.clone(),
        );
        for x in position_minimum.x..position_maximum.x {
            for y in position_minimum.y..position_maximum.y {
                for z in position_minimum.z..position_maximum.z {
                    self.grid
                        .entry((x, y, z))
                        .or_insert_with(Vec::new)
                        .push(aabb);
                    return Ok(());
                }
            }
        }
        Err(anyhow!("Error"))
    }
    /// Check if a item does colliot or not
    pub fn check_item(
        &self,
        item: Item,
        corner: &Corners,
    ) -> anyhow::Result<Option<AABBVersion1CheckedItem>> {
        let mut position_minimum = item.size_cube.clone() + corner.position.clone();
        let mut position_maximum = position_minimum.clone() + item.size_cube.clone();
        position_minimum.divide_all(self.cell_size);
        position_maximum.divide_all(self.cell_size);
        for x in position_minimum.x..position_maximum.x {
            for y in position_minimum.y..position_maximum.y {
                for z in position_minimum.z..position_maximum.z {
                    if let Some(existing) = self.grid.get(&(x, y, z)) {
                        for existing_vector6 in existing {
                            let overlay_x = position_minimum.x < existing_vector6.w
                                && existing_vector6.x < position_maximum.x;
                            let overlay_y = position_minimum.y < existing_vector6.a
                                && existing_vector6.y < position_maximum.y;
                            let overlay_z = position_minimum.z < existing_vector6.b
                                && existing_vector6.z < position_maximum.z;
                            if overlay_x && overlay_y && overlay_z {
                                return Ok(Some(AABBVersion1CheckedItem(item)));
                            }
                        }
                    }
                }
            }
        }
        Err(anyhow!("Error"))
    }
}
