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
    /// Minimum is better
    fn score(bin: &Bin, item: &Item, point: &Corners) -> f32 {
        let x = ((bin.position.x + item.size_cube.x) as f32 / (bin.position.x as f32))
            + item.order as f32;
        let y = ((bin.position.y + item.size_cube.y) as f32 / bin.position.y as f32)
            + item.weight as f32;
        let z = ((bin.position.z + item.size_cube.z) as f32 / bin.position.z as f32);
        x + y + z
    }
    /// Find best point to place
    fn find_best_point_to_place(
        points: &HashSet<Corners>,
        item: Item,
        aabb: &AABBVersion1,
        bin: &Bin,
    ) -> Option<(Option<AABBVersion1CheckedItem>, f32, Corners)> {
        let all_items_rotated = item.rotation_v2();
        all_items_rotated
            .into_par_iter()
            .filter_map(|x_item| {
                points
                    .par_iter()
                    .cloned()
                    .filter_map(|x_corner| {
                        let check = aabb.check_item(x_item.clone(), &x_corner).ok();
                        let score = Self::score(bin, &x_item, &x_corner);
                        if let Some(check) = check {
                            return Some((check, score, x_corner));
                        }
                        None
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
    ) -> anyhow::Result<ItemsPlaced> {
        let _ = aabb.add(item.clone(), &point);
        let _ = self.corners.remove(&point);
        let new_item = ItemsPlaced::new(point.position, item.0);
        Ok(new_item)
    }
}
impl Algorithmen3DBinPackaging for SecondAlgorithmen {
    fn create_algorithmen(
        input: Vec<Item>,
        bin: Bin,
    ) -> Result<Self, crate::algorithmen::AlgorithmenError> {
        let volume = bin.position.x * bin.position.y * bin.position.z;
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
                || x.size_cube.y < self.bin.position.y
                || x.size_cube.z < self.bin.position.z
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
            let result = Self::find_best_point_to_place(&self.corners, x.clone(), &aabb, &self.bin);
            if let Some(unoetig) = result
                && let Some(checked_item) = unoetig.0
            {
                let item_finished =
                    Self::place_item_in_bin(&mut self, unoetig.2, checked_item, &mut aabb)
                        .expect("Error placing");
                placed_item.push(item_finished);
            } else {
                removed_items.push(x);
            }
        });
        Ok(SortedBin::new(self.bin, placed_item))
    }
}
