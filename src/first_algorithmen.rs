use std::ops::Div;

use crate::{
    aabb::{
        AABBVersion1,
        AABBVersion1CheckedItem,
    },
    algorithmen::{
        Algorithmen3DBinPackaging,
        AlgorithmenError,
    },
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
    vector::Vector3,
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

/// The first algorithmen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmenFirst {
    items: Vec<Item>,
    Bin: Bin,
    corners: HashSet<Corners>,
    space_left: SpaceLeftBin,
    placed_item: Vec<ItemsPlaced>,
    fitness_weight: AlgorithmenFirstFitnessValues,
    collision_checker: AABBVersion1,
    removed_items_no_place: Vec<Item>,
}
/// For the evaulate where to place the different weights, if chossen wrong items can be miss placed where sub optimal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmenFirstFitnessValues {
    /// The order weight
    pub order_weight: f32,
    /// The weight for a item weight
    pub weight_weight: f32,
    /// The remaining space weight
    pub space_weight: f32,
}
impl AlgorithmenFirstFitnessValues {
    /// Constructor for AlgorithmenFirstFitnessValues  
    pub const fn new(order_weight: f32, weight_weight: f32, space_weight: f32) -> Self {
        Self {
            order_weight,
            weight_weight,
            space_weight,
        }
    }
}
impl Corners {
    /// Creates a new corner
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        // Self { x, y, z }
        Self {
            position: Vector3::new(x, y, z),
        }
    }
}
impl AlgorithmenFirst {
    /// Get the newest availbel corners for spots
    ///
    /// append the new corner on the list
    fn get_corner(bin: &Bin, item: &Item, corn: &Corners, corners: &mut HashSet<Corners>) {
        let one_corner = (
            corn.position.x + item.size_cube.x,
            corn.position.y,
            corn.position.z,
        );
        let second_corner = (
            corn.position.x,
            item.size_cube.y + corn.position.y,
            corn.position.z,
        );
        let three_corner = (
            corn.position.x,
            corn.position.y,
            item.size_cube.z + corn.position.z,
        );
        let new_corners = vec![one_corner, second_corner, three_corner];
        let new_corners: HashSet<Corners> = new_corners
            .into_par_iter()
            .filter(|x| {
                (x.0 <= bin.position.x) && (x.1 <= bin.position.y) && (x.2 <= bin.position.z)
            })
            .map(|x| Corners {
                position: Vector3::new(x.0, x.1, x.2),
            })
            .collect();
        corners.extend(new_corners);
    }
    /// Gives a index score for where to place
    ///
    /// Lower Score means better
    #[must_use]
    fn fitness_score(
        weights: &AlgorithmenFirstFitnessValues,
        bin: &Bin,
        item: &Vector3<u32>,
        space: &SpaceLeftBin,
        corner: &Corners,
        order: u32,
        weight: u32,
    ) -> f32 {
        // let space_left = weights.space_weight * (space.0 - (item.x * item.y * item.z)) as f32;
        let order = ((bin.position.x as f32)
            - ((weights.order_weight * order as f32) + item.x as f32))
            / bin.position.x as f32;
        let weight = ((bin.position.y)
            .saturating_sub(weights.weight_weight as u32 * weight + corner.position.y)
            as f32)
            .div(bin.position.y as f32);
        let first = bin.position.z.saturating_sub(item.z) as f32 / bin.position.z as f32;
        let final_result: f32 = order + weight + first;
        final_result
    }
    /// Checks best placment
    #[must_use]
    fn find_best_placment(
        bin: &Bin,
        item: &Vector3<u32>,
        corners: &HashSet<Corners>,
        space: &SpaceLeftBin,
        weights: &AlgorithmenFirstFitnessValues,
        order: u32,
        weight: u32,
        aabb: &mut AABBVersion1,
    ) -> Option<(Corners, f32, AABBVersion1CheckedItem)> {
        let mut best_corner: Option<(Corners, f32, AABBVersion1CheckedItem)> = None;
        corners.iter().for_each(|(x)| {
            let (fitness, placment) = Self::check_item(bin, item, x, space, weights, order, weight);
            let checker = match aabb.check_item(Item::new(*item, weight, order), x) {
                Ok(x) => x,
                Err(_) => None,
            };
            if let Some(check) = checker {
                if let Some(corn) = &best_corner
                    && placment
                    && fitness < corn.1 as f32
                {
                    best_corner = Some((x.clone(), fitness, check));
                } else if placment && let None = best_corner {
                    best_corner = Some((x.clone(), fitness, check));
                }
            }
        });
        return match (best_corner) {
            Some(x) => Some(x),
            _ => None,
        };
    }
    /// Places a Item in the Bin
    #[must_use]
    fn place_item(
        corner: Corners,
        bin: &mut Bin,
        item: AABBVersion1CheckedItem,
        space: &mut SpaceLeftBin,
        corner_list: &mut HashSet<Corners>,
        list_placed_items: &mut Vec<ItemsPlaced>,
        checker: &mut AABBVersion1,
    ) -> Result<(), AlgorithmenError> {
        Self::get_corner(bin, &item.0, &corner, corner_list);
        bin.weight_currently += item.0.weight;
        space.0 -= &item.0.size_cube.x * &item.0.size_cube.y * &item.0.size_cube.z;
        let new_placed_item = ItemsPlaced::new(corner.position, item.0.clone());
        // # TODO fix later
        let _ = checker
            .add(item, &corner)
            .expect("Adding to checker failed only for buildin");
        list_placed_items.push(new_placed_item);
        Ok(())
    }
    /// Checks a Item
    fn check_item(
        bin: &Bin,
        item: &Vector3<u32>,
        corner: &Corners,
        space: &SpaceLeftBin,
        weights: &AlgorithmenFirstFitnessValues,
        order: u32,
        weight: u32,
    ) -> (f32, bool) {
        let x_check = bin.position.x >= (corner.position.x + item.x);
        let y_check = bin.position.y >= (corner.position.y + item.y);
        let z_check = bin.position.z >= (corner.position.z + item.z);
        if x_check && y_check && z_check {
            let score = Self::fitness_score(weights, bin, item, space, &corner, order, weight);
            return (score, true);
        }
        (f32::MAX, false)
    }
}
impl Algorithmen3DBinPackaging for AlgorithmenFirst {
    /// Creates a new Algorithmen with the basic infos
    ///
    /// # TODO If a Item is to big for the Bin remove it
    fn create_algorithmen(input: Vec<Item>, bin: Bin) -> Result<Self, AlgorithmenError> {
        let (check, space_left) = Self::check_fit_quick(&input, &bin);
        // The output Vec needs for better performance the size pre allocated
        let items_len = input.len();
        let weight_fitenss = AlgorithmenFirstFitnessValues::new(1.0f32, 2.0f32, 1f32);
        let mut one_corner: HashSet<Corners> = HashSet::with_capacity(items_len);
        _ = one_corner.insert(Corners::new(0, 0, 0));
        if check {
            return Ok(Self {
                items: input,
                corners: one_corner,
                space_left,
                placed_item: Vec::with_capacity(items_len),
                fitness_weight: weight_fitenss,
                collision_checker: AABBVersion1::new(),
                Bin: bin,
                removed_items_no_place: Vec::new(),
            });
        } else {
            return Err(AlgorithmenError::NotEnoughSpace);
        }
    }

    /// calculates the if space is even possible to store in the bin
    fn check_fit_quick(input: &[Item], bin: &Bin) -> (bool, SpaceLeftBin) {
        let availabel_space: u32 = bin.position.x * bin.position.y * bin.position.z;
        let space_used: u32 = {
            input
                .par_iter()
                .map(|x| x.size_cube.x * x.size_cube.y * x.size_cube.z)
                .sum()
        };
        (space_used <= availabel_space, SpaceLeftBin(availabel_space))
    }

    /// Gives the final order placment in the bin with the items back
    fn calculate(mut self) -> Result<SortedBin, AlgorithmenError> {
        // Sorting after order and wheight
        //
        // weight ordering is lazy evualated
        self.items
            .sort_unstable_by(|a, b| a.order.cmp(&b.order).then_with(|| a.weight.cmp(&b.weight)));
        for item_iter in self.items.into_iter() {
            let get_all_rotations = item_iter.rotation();
            let corner = get_all_rotations
                .into_iter()
                .filter_map(|x| {
                    Self::find_best_placment(
                        &self.Bin,
                        &x,
                        &self.corners,
                        &self.space_left,
                        &self.fitness_weight,
                        item_iter.order,
                        item_iter.weight,
                        &mut self.collision_checker,
                    )
                })
                .min_by(|x, b| {
                    x.1.partial_cmp(&b.1)
                        .unwrap_or_else(|| std::cmp::Ordering::Equal)
                });
            if let Some((corner_checked, _, checked)) = corner {
                _ = self.corners.remove(&corner_checked);
                let place = Self::place_item(
                    corner_checked,
                    &mut self.Bin,
                    checked,
                    &mut self.space_left,
                    &mut self.corners,
                    &mut self.placed_item,
                    &mut self.collision_checker,
                )?;
            } else {
                self.removed_items_no_place.push(item_iter);
            }
        }
        // The Final Bin with the position Items inside
        Ok(SortedBin {
            bin: self.Bin,
            items: self.placed_item,
            removed_items: self.removed_items_no_place,
        })
    }

    fn add_item(&mut self, input: Vec<Item>) -> Result<(), AlgorithmenError> {
        let space_used: u32 = input
            .par_iter()
            .map(|x| x.size_cube.x * x.size_cube.y * x.size_cube.z)
            .sum();
        let check = self.space_left.0.saturating_sub(space_used);
        if check > 0 {
            self.items.extend(input);
            Ok(())
        } else {
            Err(AlgorithmenError::NotEnoughSpace)
        }
    }

    fn remove_item(&mut self, input: Vec<Item>) -> Result<(), AlgorithmenError> {
        for value in input.iter() {
            if let Some(index) = self.items.iter().enumerate().find(|x| x.1 == value) {
                _ = self.items.remove(index.0);
            }
        }
        Ok(())
    }

    fn space_left(&self) -> u32 {
        let availabel_space = self.Bin.position.x * self.Bin.position.y * self.Bin.position.z;
        let space_used: u32 = {
            self.items
                .par_iter()
                .map(|x| x.size_cube.x * x.size_cube.y * x.size_cube.z)
                .sum()
        };
        availabel_space - space_used
    }
}
#[cfg(test)]
mod tests {
    use hashbrown::HashSet;

    use crate::{
        algorithmen::Algorithmen3DBinPackaging,
        bin::Bin,
        first_algorithmen::AlgorithmenFirst,
        items::Item,
        vector::Vector3,
    };

    #[test]
    fn bin_fix() {
        let bin = Bin::new(Vector3::new(1000, 1000, 1000), 100000, 0);
        let item = Item::new(Vector3::new(10, 10, 10), 10, 1);
        let mut list = Vec::with_capacity(1000);
        for _ in 0..100 {
            list.push(item.clone());
        }
        let algorithmen = AlgorithmenFirst::create_algorithmen(list, bin).unwrap();
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
}
