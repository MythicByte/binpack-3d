use thiserror::Error;

use crate::{
    bin::{
        Bin,
        SpaceLeftBin,
    },
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
    fn create_algorithmen(
        input: Vec<Item>,
        bin: Bin,
    ) -> Result<AlgorithmenCreation<Self>, AlgorithmenError>;
    /// Add Items Later
    #[must_use]
    fn add_item(&mut self, input: Vec<Item>) -> Result<(), AlgorithmenError>;
    /// Remove Item
    #[must_use]
    fn remove_item(&mut self, input: Vec<Item>) -> Result<(), AlgorithmenError>;
    /// If Space is left
    fn space_left(&self) -> u32;
    /// Checks if the Items can be in a bin, possible fast check
    fn check_fit_quick(input: &[Item], bin: &Bin) -> (bool, SpaceLeftBin);
    /// A final result
    #[must_use]
    fn calculate(self) -> Result<SortedBin, AlgorithmenError>;
}

/// Errors for AlgorithmenFirst
#[derive(Debug, Error, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlgorithmenError {
    /// Bin has for the packages not enough space left {0}
    #[error("Bin has for the packages not enough space left ")]
    NotEnoughSpace,
    /// No Element was found in the list, should not be possible
    #[error("No Element was found in the list, should not be possible")]
    NoElementLeft,
}
/// For a new Algorithmen the correct response
#[derive(Debug, Clone)]
pub enum AlgorithmenCreation<T>
where
    T: Algorithmen3DBinPackaging,
{
    /// Bin could be created with items, but there where to much
    WorkedButToMuchItems {
        /// The algorithmen with items
        algorithmen: T,
        /// The items which has not been going in the bin
        items_to_much: Vec<Item>,
    },
    /// Worked with not problem
    NoProblems(T),
}
