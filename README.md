# Binpack-3D

A high-performance, deterministic 3D bin packing library written in Rust. It efficiently packs rectangular cuboids (boxes/items) into a single larger container (bin), prioritizing high-value items and maximizing space utilization.

Built with WebAssembly (Wasm) support, `binpack-3d` can be utilized natively in Rust backends or directly in the browser via TypeScript/JavaScript.

## Features

- **Single-Bin Packing**: Computes the optimal arrangement for fitting items into a single container, returning placed coordinates and a list of unplaced items.
- **Priority-First Logic**: Ensures your most important items are packed first, filling remaining space with lower-priority items.
- **Full 3D Rotation**: Supports all 6 axis-aligned orientations (permutations of width, height, and depth) to find the best fit.
- **Configurable Spacing (`gap_mm`)**: Define required spacing between items to account for padding, tolerances, or real-world physical constraints.
- **Millimeter Precision**: Uses integer-based millimeters for reliable geometry calculations without floating-point inaccuracies.
- **Wasm Ready**: First-class support for `wasm32`, allowing you to run the packing algorithms right in the browser.
- **Deterministic**: Provides consistent, repeatable results given the same inputs and configurations.

## Installation

### For Rust

Add the following to your `Cargo.toml`:

```toml
[dependencies]
binpack-3d = "0.1.0" 
```

### For JavaScript/TypeScript (Wasm)

You can use the Wasm package by importing it into your Node.js or web project once compiled with `wasm-pack`:

```bash
# Build the wasm package
wasm-pack build --target web
```

## Usage Example

### Rust

```rust
use binpack_3d::{Packer, Item, Bin};

fn main() {
  let bin = Bin::new(1,Vector3::new(1000,1000,1000),100000,0);
  let item = Item::new(1,Vector3::new(10,10,10),10,1);
  let mut item_list:Vec<Item> = Vec::with_capacity(x as usize);
  for _ in 0..x {
      item_list.push(item.clone());
  }
  let mut algorithmus = SecondAlgorithmen::create_algorithmen(item_list, bin).unwrap();
  let result = algorithmus.calculate().unwrap();
}
```

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request or open an issue.

## License
MPL 2.0
