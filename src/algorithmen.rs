use thiserror::Error;

use crate::{
    bin::{
        Bin,
        SpaceLeftBin,
    },
    corners::Corners,
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
    fn create_algorithmen(input: Vec<Item>, bin: Bin) -> Result<Self, AlgorithmenError>;
    /// Add Items Later
    #[must_use]
    fn add_item(&mut self, input: Vec<Item>) -> Result<(), AlgorithmenError>;
    /// Remove Item
    /// If not fit give back items
    #[must_use]
    fn remove_item(&mut self, input: Vec<Item>) -> Result<(), Vec<Item>>;
    /// If Space is left
    fn space_left(&self) -> u32;
    /// Checks if the Items can be in a bin, possible fast check
    fn check_fit_quick(input: &[Item], bin: &Bin) -> (bool, SpaceLeftBin);
    /// A final result
    ///
    /// the score function checks what the best position is to place a object
    ///
    /// Used default score function
    #[must_use]
    fn calculate(self) -> Result<SortedBin, AlgorithmenError>;
    /// A final result
    ///
    /// the score function checks what the best position is to place a object
    ///
    /// In some chasses a custom is preferred ['score']
    #[must_use]
    fn calculate_custom<F>(
        self,
        custom_score_function: Option<F>,
    ) -> Result<SortedBin, AlgorithmenError>
    where
        F: Fn(&Bin, &Item, &Corners) -> f32 + Send + Sync;
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
    /// Item was to big for item
    #[error("Item was to big for item")]
    ItemToBigForBin,
}
