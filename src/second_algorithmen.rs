use std::mem;

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
    bin::Bin,
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
        let x = (point.position.x + item.size_cube.x) as f32 + 1.0 / (bin.position.x as f32) + 1.0;
        let y = (point.position.y + item.size_cube.y) as f32 + 1.0 / (bin.position.y as f32) + 1.0;
        let z = (point.position.z + item.size_cube.z) as f32 + 1.0 / (bin.position.z as f32) + 1.0;
        (x * 10.0) + (y * 100.0) + z
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
            .into_par_iter()
            .filter_map(|x_item| {
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
                        let check = aabb.check_item(x_item.clone(), &x_corner).ok().flatten()?;
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
        let _ = aabb.add(item.clone(), &point);
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

    fn remove_item(
        &mut self,
        input: Vec<Item>,
    ) -> Result<(), crate::algorithmen::AlgorithmenError> {
        todo!()
    }

    fn space_left(&self) -> u32 {
        todo!()
    }

    fn check_fit_quick(input: &[Item], bin: &Bin) -> (bool, crate::bin::SpaceLeftBin) {
        todo!()
    }

    fn calculate(
        mut self,
    ) -> Result<crate::sortedbin::SortedBin, crate::algorithmen::AlgorithmenError> {
        let item = mem::take(&mut self.items);
        let (mut keep, remove): (Vec<Item>, Vec<Item>) = item.into_par_iter().partition(|x| {
            x.size_cube.x < self.bin.position.x
                && x.size_cube.y < self.bin.position.y
                && x.size_cube.z < self.bin.position.z
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
                let (item_finished, new_corners) = Self::place_item_in_bin(
                    &mut self,
                    checked_result.2,
                    checked_result.0,
                    &mut aabb,
                )
                .expect("Error placing");
                placed_item.push(item_finished);
                self.corners.extend(new_corners);
            } else {
                removed_items.push(x);
            }
        });
        Ok(SortedBin::new(self.bin, placed_item, removed_items))
    }
}
#[cfg(test)]
mod tests {
    use hashbrown::HashSet;
    use proptest::prelude::*;

    use crate::{
        algorithmen::Algorithmen3DBinPackaging,
        bin::Bin,
        corners::Corners,
        items::Item,
        second_algorithmen::SecondAlgorithmen,
        vector::Vector3,
    };

    #[test]
    fn bin_fix_v2() {
        let bin = Bin::new(Vector3::new(1000, 1000, 1000), 100000, 0);
        let item = Item::new(Vector3::new(10, 10, 10), 10, 1);
        let mut list = Vec::with_capacity(1000);
        for _ in 0..100 {
            list.push(item.clone());
        }
        let algorithmen = SecondAlgorithmen::create_algorithmen(list, bin).unwrap();
        let result = algorithmen.calculate().unwrap();
        assert_eq!(100, result.items.len());
        assert_eq!(0, result.removed_items.len());
        let mut hash_check: HashSet<Vector3<u32>> = HashSet::with_capacity(result.items.len());
        for i in result.items {
            if !hash_check.insert(i.position) {
                panic!("Same corner was used");
            }
        }
    }
    #[test]
    fn bin_variable_v3() {
        let x = || return rand::random_range(0u32..100u32);
        let bin = Bin::new(Vector3::new(10000, 10000, 10000), 100000, 0);
        let item = Item::new(Vector3::new(x(), x(), x()), 10, 1);
        let mut list = Vec::with_capacity(1000);
        for _ in 0..100 {
            list.push(item.clone());
        }
        let algorithmen = SecondAlgorithmen::create_algorithmen(list, bin).unwrap();
        let result = algorithmen.calculate().unwrap();
        assert_eq!(100, result.items.len());
        assert_eq!(0, result.removed_items.len());
        let mut hash_check: HashSet<Vector3<u32>> = HashSet::with_capacity(result.items.len());
        for i in result.items {
            if !hash_check.insert(i.position) {
                panic!("Same corner was used");
            }
        }
    }
    proptest! {
        #[test]
        fn second_algorithmen_score(x in 0u32..100,y in 0u32..100,z in 0u32..100) {
            let result = SecondAlgorithmen::score(&Bin::new(Vector3::new(x + 100, y + 100, z + 100), 1000, 0), &Item::new(Vector3::new(x, y, z), 10, 1), &Corners::new(0,0,0));
            dbg!(&result);
            prop_assert!(result >= 0.0);
            prop_assert!(!result.is_nan());
            }
    }
}
