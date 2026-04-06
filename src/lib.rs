//! A programm for sorting items in bins
#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![deny(unused_results)]
/// A AABB Checker
pub mod aabb;
/// The generic trait for the algorithmens
pub mod algorithmen;
/// A bin for items
pub mod bin;
/// The corner
pub mod corners;
/// The first algorithmen for placing items in a bin
#[cfg(feature = "first_algorithmen")]
pub mod first_algorithmen;
/// A item
pub mod items;
/// Second algorithmen
#[cfg(feature = "second_algorithmen")]
pub mod second_algorithmen;
/// Finished Bin
pub mod sortedbin;
/// Vectors
pub mod vector;
/// Wasm interface
#[cfg(target_arch = "wasm32")]
pub mod wasm_interface;
