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
#[derive(Debug, Error)]
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

/// Converts a u32 which must be in mm converts it to, mm, cm and meter
macro_rules! convert_mm_to_meter_cm_mm {
    ($input:expr) => {{
        let input: u32 = $input;
        let (mut meter, mut cm, mut mm): (u32, u32, u32) = (0, 0, 0);
        mm = input % 10;
        cm = (input / 10) % 10;
        meter = input / 100;
        (meter, cm, mm)
    }};
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;
    use proptest::prelude::*;
    #[test]
    fn check_macro() {
        let values = convert_mm_to_meter_cm_mm!(97834);
        assert_eq!(97834, (values.0 * 100 + values.1 * 10 + values.2));
    }
    #[test]
    fn check_macro_insta() {
        let value = convert_mm_to_meter_cm_mm!(12435534);
        assert_yaml_snapshot!(value);
    }
    proptest! {
        #[allow(unused_parens)]
        #[test]
        fn check_macro_proptest(input in any::<u32>()) {
            let value = convert_mm_to_meter_cm_mm!(input);
            prop_assert_eq!(input,(value.0 * 100 + value.1 * 10 + value.2));
        }
    }
}
