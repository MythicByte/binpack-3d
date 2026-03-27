use crate::{algorithmen::Algorithmen3DBinPackaging, bin::Bin, items::Item};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    prelude,
};

/// The first algorithmen
#[derive(Debug)]
pub struct AlgorithmenFirst {
    items: Vec<Item>,
    Bin: Bin,
    corners: Vec<Corners>,
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
impl Corners {
    /// Creates a new corner
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
impl AlgorithmenFirst {
    fn get_corner(&mut self) {
        todo!()
    }
    /// Gives a index score for where to place
    fn fitness_score(&self) -> usize {
        todo!()
    }
    /// Checks best placment
    fn find_best_placment(&self) {
        todo!()
    }
}
impl Algorithmen3DBinPackaging for AlgorithmenFirst {
    fn give_offline(input: Vec<Item>, bin: Bin) -> Result<Self, Box<dyn std::error::Error>> {
        if Self::check_fit_quick(&input, &bin) {
            return Ok(Self {
                items: input,
                Bin: bin,
                corners: vec![Corners::new(0f32, 0f32, 0f32)],
            });
        }
        todo!()
    }

    fn check_fit_quick(input: &[Item], bin: &Bin) -> bool {
        let availabel_space = bin.heigth * bin.width * bin.length;
        let space_used: f32 = {
            input
                .par_iter()
                .map(|x| x.heigth * x.width * x.length)
                .sum()
        };
        space_used <= availabel_space
    }

    fn calculate(&mut self) -> Result<crate::sortedbin::SortedBin, Box<dyn std::error::Error>> {
        todo!()
    }
}
