use crate::{bin::Bin, items::Item, sortedbin::SortedBin};

/// Defines a trait for 3d Bin Packaging algorithments, so is replacing the algorithm possible
pub trait Algorithmen3DBinPackaging
where
    Self: Sized,
{
    /// A Algorithmen Input where all packages are there
    fn give_offline(input: Vec<Item>, bin: Bin) -> Result<Self, Box<dyn std::error::Error>>;
    // For later
    // fn give_online(input: Vec<Items>, bin: Bin) -> Result<SortedBin, Box<dyn std::error::Error>>;
    /// Checks if the Items can be in a bin, possible fast check
    fn check_fit_quick(input: &[Item], bin: &Bin) -> bool;
    /// A final result
    fn calculate(&mut self) -> Result<SortedBin, Box<dyn std::error::Error>>;
}
