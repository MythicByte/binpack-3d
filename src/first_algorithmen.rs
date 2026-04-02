use crate::{
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
};
use hashbrown::HashSet;
use nalgebra::Vector3;
use rayon::iter::{
    IntoParallelIterator,
    IntoParallelRefIterator,
    ParallelIterator,
};

/// The first algorithmen
#[derive(Debug, Clone)]
pub struct AlgorithmenFirst {
    items: Vec<Item>,
    Bin: Bin,
    corners: HashSet<Corners>,
    space_left: SpaceLeftBin,
    placed_item: Vec<ItemsPlaced>,
    fitness_weight: AlgorithmenFirstFitnessValues,
}
/// For the evaulate where to place the different weights, if chossen wrong items can be miss placed where sub optimal
#[derive(Debug, Clone)]
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
            corn.position.x + item.position.x,
            corn.position.y,
            corn.position.z,
        );
        let second_corner = (
            corn.position.x,
            item.position.y + corn.position.y,
            corn.position.z,
        );
        let three_corner = (
            corn.position.x,
            corn.position.y,
            item.position.z + corn.position.z,
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
        item: &Item,
        space: &SpaceLeftBin,
        corner: &Corners,
    ) -> u32 {
        let space_left: u32 = (weights.space_weight
            * (space.0 - (item.position.x * item.position.y * item.position.z)) as f32)
            as u32;
        let order = (weights.order_weight * item.order as f32) as u32;
        let weight = (weights.weight_weight * item.weight as f32) as u32;
        let height = item.position.z + corner.position.z;
        // Downcasting the rounding errros are ignored
        let final_result: u32 = space_left + order + weight + height;
        final_result
    }
    /// Checks best placment
    #[must_use]
    fn find_best_placment(
        bin: &Bin,
        item: &Item,
        corners: &HashSet<Corners>,
        space: &SpaceLeftBin,
        weights: &AlgorithmenFirstFitnessValues,
    ) -> Option<(Corners, usize)> {
        let mut best_corner: Option<(Corners, u32, usize)> = None;
        corners.iter().enumerate().for_each(|(index, x)| {
            let (fitness, placment) = Self::check_item(bin, item, x, space, weights);
            if let Some(corn) = &best_corner
                && placment
                && fitness > corn.1
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
        space.0 -= &item.position.x * &item.position.y * &item.position.z;
        let new_placed_item = ItemsPlaced::new(
            corner.position.x,
            corner.position.y,
            corner.position.z,
            item,
        );
        list_placed_items.push(new_placed_item);
        Ok(())
    }
    /// Checks a Item
    fn check_item(
        bin: &Bin,
        item: &Item,
        corner: &Corners,
        space: &SpaceLeftBin,
        weights: &AlgorithmenFirstFitnessValues,
    ) -> (u32, bool) {
        let x_check = bin.position.x >= (corner.position.x + item.position.x);
        let y_check = bin.position.y >= (corner.position.y + item.position.y);
        let z_check = bin.position.z >= (corner.position.z + item.position.z);
        if x_check && y_check && z_check {
            let score = Self::fitness_score(weights, bin, item, space, &corner);
            return (score, true);
        }
        (u32::MAX, false)
    }
}
impl Algorithmen3DBinPackaging for AlgorithmenFirst {
    /// Creates a new Algorithmen with the basic infos
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
                Bin: bin,
                corners: one_corner,
                space_left: space_left,
                placed_item: Vec::with_capacity(items_len),
                fitness_weight: weight_fitenss,
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
                .map(|x| x.position.x * x.position.y * x.position.z)
                .sum()
        };
        (space_used <= availabel_space, SpaceLeftBin(availabel_space))
    }

    /// Gives the final order placment in the bin with the items back
    ///
    /// TODO: The Items can be rotated, needs to be implemented
    fn calculate(mut self) -> Result<SortedBin, AlgorithmenError> {
        // Sorting after order and wheight
        //
        // weight ordering is lazy evualated
        self.items
            .sort_unstable_by(|a, b| a.order.cmp(&b.order).then_with(|| a.weight.cmp(&b.weight)));
        for item_iter in self.items.into_iter() {
            let corner = Self::find_best_placment(
                &self.Bin,
                &item_iter,
                &self.corners,
                &self.space_left,
                &self.fitness_weight,
            );
            if let Some((corner_checked, _index)) = corner {
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
            .map(|x| x.position.x * x.position.y * x.position.z)
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
                .map(|x| x.position.x * x.position.y * x.position.z)
                .sum()
        };
        availabel_space - space_used
    }
}
