use crate::{
    aabb::AABBVersion1,
    algorithmen::{
        Algorithmen3DBinPackaging,
        AlgorithmenCreation,
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
use wasm_bindgen::prelude::wasm_bindgen;

/// The first algorithmen
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmenFirst {
    items: Vec<Item>,
    Bin: Bin,
    corners: HashSet<Corners>,
    space_left: SpaceLeftBin,
    placed_item: Vec<ItemsPlaced>,
    fitness_weight: AlgorithmenFirstFitnessValues,
    collision_checker: AABBVersion1,
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
        let space_left = weights.space_weight * (space.0 - (item.x * item.y * item.z)) as f32;
        let order = weights.order_weight * order as f32;
        let weight = weights.weight_weight * weight as f32;
        let height = (item.z + corner.position.z) as f32;
        // Downcasting the rounding errros are ignored
        let final_result: f32 = space_left + order + weight + height;
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
    ) -> Option<(Corners, usize)> {
        let mut best_corner: Option<(Corners, f32, usize)> = None;
        corners.iter().enumerate().for_each(|(index, x)| {
            let (fitness, placment) = Self::check_item(bin, item, x, space, weights, order, weight);
            if let Some(corn) = &best_corner
                && placment
                && fitness > corn.1 as f32
            {
                best_corner = Some((x.clone(), fitness, index));
            } else if placment && let None = best_corner {
                best_corner = Some((x.clone(), fitness, index));
            }
        });
        return match best_corner {
            Some(x) => Some((x.0, x.2)),
            None => None,
        };
    }
    /// Places a Item in the Bin
    #[must_use]
    fn place_item(
        corner: Corners,
        bin: &mut Bin,
        item: Item,
        space: &mut SpaceLeftBin,
        corner_list: &mut HashSet<Corners>,
        list_placed_items: &mut Vec<ItemsPlaced>,
    ) -> Result<(), AlgorithmenError> {
        Self::get_corner(bin, &item, &corner, corner_list);
        bin.weight_currently += item.weight;
        space.0 -= &item.size_cube.x * &item.size_cube.y * &item.size_cube.z;
        let new_placed_item = ItemsPlaced::new(corner.position, item);
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
    fn create_algorithmen(
        input: Vec<Item>,
        bin: Bin,
    ) -> Result<AlgorithmenCreation<Self>, AlgorithmenError> {
        let (check, space_left) = Self::check_fit_quick(&input, &bin);
        // The output Vec needs for better performance the size pre allocated
        let items_len = input.len();
        let weight_fitenss = AlgorithmenFirstFitnessValues::new(1f32, 1f32, 1f32);
        let mut one_corner: HashSet<Corners> = HashSet::with_capacity(items_len);
        _ = one_corner.insert(Corners::new(0, 0, 0));
        if check {
            return Ok(AlgorithmenCreation::NoProblems(Self {
                items: input,
                corners: one_corner,
                space_left,
                placed_item: Vec::with_capacity(items_len),
                fitness_weight: weight_fitenss,
                collision_checker: AABBVersion1::new(bin.clone()),
                Bin: bin,
            }));
        } else {
            return Err(AlgorithmenError::NotEnoughSpace);
        }
    }

    /// calculates the if space is even possible to store in the bin
    fn check_fit_quick(input: &[Item], bin: &Bin) -> (bool, SpaceLeftBin) {
        let availabel_space = bin.position.x * bin.position.y * bin.position.z;
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
                .into_par_iter()
                .filter_map(|x| {
                    Self::find_best_placment(
                        &self.Bin,
                        &x,
                        &self.corners,
                        &self.space_left,
                        &self.fitness_weight,
                        item_iter.order,
                        item_iter.weight,
                    )
                })
                .min_by_key(|x| x.1);
            if let Some((corner_checked, index)) = corner {
                _ = self.corners.remove(&corner_checked);
                let place = Self::place_item(
                    corner_checked,
                    &mut self.Bin,
                    item_iter,
                    &mut self.space_left,
                    &mut self.corners,
                    &mut self.placed_item,
                )?;
            } else {
                return Err(AlgorithmenError::NotEnoughSpace);
            }
        }
        // The Final Bin with the position Items inside
        Ok(SortedBin {
            bin: self.Bin,
            items: self.placed_item,
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
