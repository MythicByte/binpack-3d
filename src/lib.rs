//! A programm for sorting items in bins
#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![deny(unused_results)]
/// The generic trait for the algorithmens
pub mod algorithmen;
/// A bin for items
pub mod bin;
/// The first algorithmen for placing items in a bin
pub mod first_algorithmen;
/// A item
pub mod items;
/// Finished Bin
pub mod sortedbin;
