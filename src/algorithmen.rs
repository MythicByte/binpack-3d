use thiserror::Error;

use crate::{
    bin::Bin,
    items::Item,
    sortedbin::SortedBin,
};

/// Defines a trait for 3d Bin Packaging algorithments, so is replacing the algorithm possible
pub trait Algorithmen3DBinPackaging
where
    Self: Sized,
{
    /// A Algorithmen Input where all packages are there
    #[must_use]
    fn give_offline(input: Vec<Item>, bin: Bin) -> Result<Self, AlgorithmenError>;
    // For later
    // fn give_online(input: Vec<Items>, bin: Bin) -> Result<SortedBin, Box<dyn std::error::Error>>;
    /// Checks if the Items can be in a bin, possible fast check
    fn check_fit_quick(input: &[Item], bin: &Bin) -> (bool, SpaceLeftBin);
    /// A final result
    #[must_use]
    fn calculate(self) -> Result<SortedBin, AlgorithmenError>;
}

/// Errors for AlgorithmenFirst
#[derive(Debug, Error)]
pub enum AlgorithmenError {
    /// Bin has for the packages not enough space left {0}
    #[error("Bin has for the packages not enough space left ")]
    NotEnoughSpace,
    /// No Element was found in the list, should not be possible
    #[error("No Element was found in the list, should not be possible")]
    NoElementLeft,
}
/// Gives for a Bin Space back
#[derive(Debug)]
pub struct SpaceLeftBin(pub f32);
