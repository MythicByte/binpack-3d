use crate::{
    algorithmen::{
        Algorithmen3DBinPackaging,
        AlgorithmenError,
        SpaceLeftBin,
    },
    bin::Bin,
    items::{
        Item,
        ItemsPlaced,
    },
    sortedbin::SortedBin,
};
use rayon::iter::{
    IntoParallelRefIterator,
    ParallelIterator,
};

/// The first algorithmen
#[derive(Debug)]
pub struct AlgorithmenFirst {
    items: Vec<Item>,
    Bin: Bin,
    corners: Vec<Corners>,
    space_left: SpaceLeftBin,
    placed_item: Vec<ItemsPlaced>,
    fitness_weight: AlgorithmenFirstFitnessValues,
}
/// A item corner
#[derive(Debug)]
pub struct Corners {
    /// x
    pub x: f32,
    /// y
    pub y: f32,
    /// z
    pub z: f32,
}
/// For the evaulate where to place the different weights, if chossen wrong items can be miss placed where sub optimal
#[derive(Debug)]
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
    pub fn new(order_weight: f32, weight_weight: f32, space_weight: f32) -> Self {
        Self {
            order_weight,
            weight_weight,
            space_weight,
        }
    }
}
impl Corners {
    /// Creates a new corner
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
impl AlgorithmenFirst {
    /// Get the newest availbel corners for spots
    fn get_corner(&mut self) {
        todo!()
    }
    /// Gives a index score for where to place
    #[must_use]
    fn fitness_score(&self) -> Result<usize, AlgorithmenError> {
        let first_item = match self.items.first() {
            Some(x) => x,
            None => return Err(AlgorithmenError::NoElementLeft),
        };
        let space = self.fitness_weight.space_weight * self.space_left.0;
        let order = self.fitness_weight.order_weight * first_item.order as f32;
        let weight = self.fitness_weight.weight_weight * first_item.weight;
        // Downcasting the rounding errros are ignored
        let final_result: usize = (0f32 + space + order + weight) as usize;
        Ok(final_result)
    }
    /// Checks best placment
    #[must_use]
    fn find_best_placment(bin: &Bin, item: &Item, corners: &Vec<Corners>) -> Option<Corners> {
        todo!()
    }
    /// Places a Item in the Bin
    #[must_use]
    fn place_item(
        corner: Corners,
        bin: &Bin,
        item: &Item,
        space: &mut SpaceLeftBin,
    ) -> Result<(), AlgorithmenError> {
        todo!()
    }
}
impl Algorithmen3DBinPackaging for AlgorithmenFirst {
    /// Creates a new Algorithmen with the basic infos
    fn give_offline(input: Vec<Item>, bin: Bin) -> Result<Self, AlgorithmenError> {
        let (check, space_left) = Self::check_fit_quick(&input, &bin);
        // The output Vec needs for better performance the size pre allocated
        let items_len = input.len();
        let weight_fitenss = AlgorithmenFirstFitnessValues::new(1f32, 1f32, 1f32);
        if check {
            return Ok(Self {
                items: input,
                Bin: bin,
                corners: vec![Corners::new(0f32, 0f32, 0f32)],
                space_left: space_left,
                placed_item: Vec::with_capacity(items_len),
                fitness_weight: weight_fitenss,
            });
        } else {
            return Err(AlgorithmenError::NotEnoughSpace);
        }
    }

    /// calculates the if space is even possible to store in the bin
    fn check_fit_quick(input: &[Item], bin: &Bin) -> (bool, SpaceLeftBin) {
        let availabel_space = bin.heigth * bin.width * bin.length;
        let space_used: f32 = {
            input
                .par_iter()
                .map(|x| x.heigth * x.width * x.length)
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
        self.items.sort_unstable_by(|a, b| {
            a.order
                .cmp(&b.order)
                .then_with(|| a.weight.total_cmp(&b.weight))
        });
        for item_iter in self.items.iter_mut() {
            let corner = Self::find_best_placment(&self.Bin, &item_iter, &self.corners);
            if let Some(corner_checked) = corner {
                let place =
                    Self::place_item(corner_checked, &self.Bin, &item_iter, &mut self.space_left)?;
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
}
