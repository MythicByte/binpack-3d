use std::{
    mem,
    u32,
};

use hashbrown::HashSet;
use rayon::iter::{
    IntoParallelIterator,
    IntoParallelRefIterator,
    ParallelIterator,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    aabb::{
        AABBVersion1,
        AABBVersion1CheckedItem,
    },
    algorithmen::Algorithmen3DBinPackaging,
    bin::{
        Bin,
        SpaceLeftBin,
    },
    corners::Corners,
    items::{
        Item,
        ItemsPlaced,
    },
    sortedbin::SortedBin,
};

/// Second algorithmen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondAlgorithmen {
    bin: Bin,
    items: Vec<Item>,
    aabb: AABBVersion1,
    /// Volume left in the bin
    volume_left: u32,
    corners: HashSet<Corners>,
}
impl SecondAlgorithmen {
    fn is_support_under_it(items_placed: &Vec<ItemsPlaced>, item: &Item, point: &Corners) -> bool {
        // on the ground
        if point.position.y == 0 {
            return true;
        }
        // Candidate footprint (half-open [min, max))
        let min_x = point.position.x;
        let max_x = point.position.x + item.size_cube.x;
        let min_z = point.position.z;
        let max_z = point.position.z + item.size_cube.z;

        items_placed.iter().any(|p| {
            // Must sit exactly on top of this item
            let top_y = p.position.y + p.item.size_cube.y;
            if top_y != point.position.y {
                return false;
            }

            // Support item's footprint
            let p_min_x = p.position.x;
            let p_max_x = p.position.x + p.item.size_cube.x;
            let p_min_z = p.position.z;
            let p_max_z = p.position.z + p.item.size_cube.z;

            let overlap_x = min_x < p_max_x && p_min_x < max_x;
            let overlap_z = min_z < p_max_z && p_min_z < max_z;

            overlap_x && overlap_z
        })
    }
    /// if it fits in the bin
    fn fits_in_bin(bin: &Bin, corner: &Corners, item: &Item) -> bool {
        corner.position.x + item.size_cube.x <= bin.position.x
            && corner.position.y + item.size_cube.y <= bin.position.y
            && corner.position.z + item.size_cube.z <= bin.position.z
    }
    /// Minimum is better
    fn score(bin: &Bin, item: &Item, point: &Corners) -> f32 {
        // without + 1 NaN can be possible
        let x =
            ((point.position.x + item.size_cube.x) as f32 + 1.0) / (bin.position.x as f32) + 1.0;
        let y =
            ((point.position.y + item.size_cube.y) as f32 + 1.0) / (bin.position.y as f32) + 1.0;
        let z =
            ((point.position.z + item.size_cube.z) as f32 + 1.0) / (bin.position.z as f32) + 1.0;
        let maximise_space_on_floor = (item.size_cube.x * item.size_cube.z) as f32;
        let result =
            ((x) + (y * 100.0) + (10.0 * z) - maximise_space_on_floor).clamp(f32::MIN, f32::MAX);
        result
    }
    /// Find best point to place
    fn find_best_point_to_place(
        points: &HashSet<Corners>,
        item: Item,
        aabb: &AABBVersion1,
        bin: &Bin,
        placed_items: &Vec<ItemsPlaced>,
    ) -> Option<(AABBVersion1CheckedItem, f32, Corners)> {
        let all_items_rotated = item.rotation_v2();
        all_items_rotated
            .into_iter()
            .filter_map(|x_item| {
                // rayon removed for the second iterator check later
                points
                    .par_iter()
                    .cloned()
                    .filter_map(|x_corner| {
                        // if outer bounds ignore
                        if !Self::fits_in_bin(bin, &x_corner, &x_item)
                            || !Self::is_support_under_it(placed_items, &x_item, &x_corner)
                        {
                            return None;
                        }
                        let check = aabb.check_item_v2(x_item.clone(), &x_corner)?;
                        let score = Self::score(bin, &x_item, &x_corner);
                        return Some((check, score, x_corner));
                    })
                    .min_by(|a, b| {
                        a.1.partial_cmp(&b.1)
                            .unwrap_or_else(|| std::cmp::Ordering::Equal)
                    })
            })
            .min_by(|a, b| {
                a.1.partial_cmp(&b.1)
                    .unwrap_or_else(|| std::cmp::Ordering::Equal)
            })
    }
    /// place item in bin
    fn place_item_in_bin(
        &mut self,
        point: Corners,
        item: crate::aabb::AABBVersion1CheckedItem,
        aabb: &mut AABBVersion1,
    ) -> anyhow::Result<(ItemsPlaced, Vec<Corners>)> {
        let _ = aabb.add_v2(item.clone(), &point);
        let _ = self.corners.remove(&point);
        let one_corner = Corners::new(
            point.position.x + item.0.size_cube.x,
            point.position.y,
            point.position.z,
        );
        let second_pointer = Corners::new(
            point.position.x,
            item.0.size_cube.y + point.position.y,
            point.position.z,
        );
        let three_pointer = Corners::new(
            point.position.x,
            point.position.y,
            item.0.size_cube.z + point.position.z,
        );
        let new_corners = vec![one_corner, second_pointer, three_pointer];
        // Checks if a corner is in the bin
        let new_corners = new_corners
            .into_iter()
            .filter_map(|x| aabb.point_is_free(&x).then(|| x))
            .collect();
        let new_item = ItemsPlaced::new(point.position, item.0);
        Ok((new_item, new_corners))
    }
}
impl Algorithmen3DBinPackaging for SecondAlgorithmen {
    fn create_algorithmen(
        input: Vec<Item>,
        bin: Bin,
    ) -> Result<Self, crate::algorithmen::AlgorithmenError> {
        let volume = bin
            .position
            .x
            .saturating_mul(bin.position.y)
            .saturating_mul(bin.position.z);
        let length = input.len();
        let mut hashset: HashSet<Corners> = HashSet::with_capacity(length);
        // ignore
        let _ = hashset.insert(Corners::new(0, 0, 0));
        Ok(Self {
            bin,
            items: input,
            aabb: AABBVersion1::new(),
            volume_left: volume,
            corners: hashset,
        })
    }

    fn add_item(&mut self, input: Vec<Item>) -> Result<(), crate::algorithmen::AlgorithmenError> {
        todo!()
    }

    fn remove_item(&mut self, input: Vec<Item>) -> Result<(), Vec<Item>> {
        todo!();
    }

    fn space_left(&self) -> u32 {
        self.volume_left
    }

    fn check_fit_quick(input: &[Item], bin: &Bin) -> (bool, crate::bin::SpaceLeftBin) {
        let bin_volume =
            (bin.position.x * bin.position.y * bin.position.z).clamp(u32::MIN, u32::MAX);
        let item_total_volume: u32 = input
            .iter()
            .map(|x| x.size_cube.x * x.size_cube.y * x.size_cube.z)
            .sum();

        let item_total_volume = item_total_volume.clamp(u32::MIN, u32::MAX);
        let result = (bin_volume - item_total_volume).clamp(u32::MIN, u32::MAX);
        let result = SpaceLeftBin(result);
        let check = result.0 > 0;
        (check, result)
    }

    fn calculate(
        mut self,
    ) -> Result<crate::sortedbin::SortedBin, crate::algorithmen::AlgorithmenError> {
        let item = mem::take(&mut self.items);
        let (mut keep, remove): (Vec<Item>, Vec<Item>) = item.into_par_iter().partition(|x| {
            x.size_cube.x <= self.bin.position.x
                && x.size_cube.y <= self.bin.position.y
                && x.size_cube.z <= self.bin.position.z
        });
        keep.sort_unstable_by(|a, b| a.order.cmp(&b.order).then_with(|| a.weight.cmp(&b.weight)));
        // takes item
        let item: Vec<Item> = keep;
        let mut removed_items: Vec<Item> = Vec::with_capacity(item.len());
        removed_items.extend(remove);
        let mut placed_item: Vec<ItemsPlaced> = Vec::with_capacity(item.len());

        // Takes checker
        let mut aabb = mem::take(&mut self.aabb);
        item.into_iter().for_each(|x| {
            let result = Self::find_best_point_to_place(
                &self.corners,
                x.clone(),
                &aabb,
                &self.bin,
                &placed_item,
            );
            if let Some(checked_result) = result {
                if let Ok((item_finished, new_corners)) = Self::place_item_in_bin(
                    &mut self,
                    checked_result.2,
                    checked_result.0,
                    &mut aabb,
                ) {
                    self.volume_left = self
                        .volume_left
                        .saturating_sub(item_finished.item.volume_item());
                    placed_item.push(item_finished);
                    self.corners.extend(new_corners);
                } else {
                    removed_items.push(x);
                }
            } else {
                removed_items.push(x);
            }
        });
        Ok(SortedBin::new(self.bin, placed_item, removed_items))
    }
}
#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::{
        bin::Bin,
        corners::Corners,
        items::Item,
        second_algorithmen::SecondAlgorithmen,
        vector::Vector3,
    };

    proptest! {
        #[test]
        fn second_algorithmen_score(x in 0u32..100,y in 0u32..100,z in 0u32..100) {
            let result = SecondAlgorithmen::score(&Bin::new(Vector3::new(x + 100, y + 100, z + 100), 1000, 0), &Item::new(Vector3::new(x, y, z), 10, 1), &Corners::new(0,0,0));
            dbg!(&result);
            prop_assert!(result.is_normal());
            prop_assert!(!result.is_nan());
            }
    }
}
