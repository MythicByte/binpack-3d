use wasm_bindgen::prelude::*;

use crate::{
    algorithmen::{
        Algorithmen3DBinPackaging,
        AlgorithmenError,
    },
    bin::Bin,
    items::{
        Item,
        ItemsPlaced,
    },
    second_algorithmen::SecondAlgorithmen,
    vector::Vector3,
};

fn to_js_error(e: AlgorithmenError) -> JsError {
    JsError::new(&e.to_string())
}

///
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct BinSpec {
    x: u32,
    y: u32,
    z: u32,
    max_weight: u32,
    weight_currently: u32,
}

#[wasm_bindgen]
impl BinSpec {
    #[wasm_bindgen(constructor)]
    ///
    pub fn new(x: u32, y: u32, z: u32, max_weight: u32, weight_currently: u32) -> BinSpec {
        Self {
            x,
            y,
            z,
            max_weight,
            weight_currently,
        }
    }

    #[wasm_bindgen(getter)]
    ///
    pub fn x(&self) -> u32 {
        self.x
    }
    #[wasm_bindgen(getter)]
    ///
    pub fn y(&self) -> u32 {
        self.y
    }
    #[wasm_bindgen(getter)]
    ///
    pub fn z(&self) -> u32 {
        self.z
    }
    #[wasm_bindgen(getter)]
    ///
    pub fn max_weight(&self) -> u32 {
        self.max_weight
    }
    #[wasm_bindgen(getter)]
    ///
    pub fn weight_currently(&self) -> u32 {
        self.weight_currently
    }
}

impl From<BinSpec> for Bin {
    fn from(b: BinSpec) -> Self {
        Bin::new(
            Vector3::new(b.x, b.y, b.z),
            b.max_weight,
            b.weight_currently,
        )
    }
}

///
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct ItemSpec {
    x: u32,
    y: u32,
    z: u32,
    weight: u32,
    order: u32,
}

///
#[wasm_bindgen]
impl ItemSpec {
    #[wasm_bindgen(constructor)]
    ///
    pub fn new(x: u32, y: u32, z: u32, weight: u32, order: u32) -> ItemSpec {
        Self {
            x,
            y,
            z,
            weight,
            order,
        }
    }

    #[wasm_bindgen(getter)]
    ///
    pub fn x(&self) -> u32 {
        self.x
    }
    #[wasm_bindgen(getter)]
    ///
    pub fn y(&self) -> u32 {
        self.y
    }
    #[wasm_bindgen(getter)]
    ///
    pub fn z(&self) -> u32 {
        self.z
    }
    #[wasm_bindgen(getter)]
    ///
    pub fn weight(&self) -> u32 {
        self.weight
    }
    #[wasm_bindgen(getter)]
    ///
    pub fn order(&self) -> u32 {
        self.order
    }
}

impl From<ItemSpec> for Item {
    fn from(i: ItemSpec) -> Self {
        Item::new(Vector3::new(i.x, i.y, i.z), i.weight, i.order)
    }
}

/// A minimal wasm-safe calculation result.
/// (Extend this later if you want to return full placement info.)
#[wasm_bindgen]
pub struct CalcResult {
    /// x
    pub x: u32,
    /// y
    pub y: u32,
    /// z
    pub z: u32,
    /// x
    pub size_x: u32,
    /// y
    pub size_y: u32,
    /// z
    pub size_z: u32,
    /// ignore
    pub ignore: usize,
}

#[wasm_bindgen]
impl CalcResult {
    #[wasm_bindgen(constructor)]
    ///
    pub fn new(x: u32, y: u32, z: u32, size_x: u32, size_y: u32, size_z: u32) -> Self {
        Self {
            x,
            y,
            z,
            size_x,
            size_y,
            size_z,
            ignore: 0,
        }
    }
    #[wasm_bindgen]
    /// Center point
    pub fn center_point(&self) -> Center {
        let x = (self.x + self.size_x) / 2;
        let y = (self.y + self.size_y) / 2;
        let z = (self.z + self.size_z) / 2;
        Center::new(x, y, z)
    }
}
/// Center
#[wasm_bindgen]
pub struct Center {
    /// x
    pub x: u32,
    /// y
    pub y: u32,
    /// z
    pub z: u32,
}
impl Center {
    /// Constructor
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }
}
impl From<ItemsPlaced> for CalcResult {
    fn from(value: ItemsPlaced) -> Self {
        Self {
            x: value.position.x,
            y: value.position.y,
            z: value.position.z,
            size_x: value.item.size_cube.x,
            size_y: value.item.size_cube.y,
            size_z: value.item.size_cube.z,
            ignore: 0,
        }
    }
}
/// WASM-exported wrapper around your internal algorithm.
/// This avoids wasm-bindgen needing to understand AlgorithmenFirst's internal fields.
#[wasm_bindgen]
pub struct AlgorithmenFirstWasm {
    inner: SecondAlgorithmen,
}

#[wasm_bindgen]
impl AlgorithmenFirstWasm {
    /// Create algorithm from JS:
    /// - `input`: array of ItemSpec
    /// - `bin`: BinSpec
    #[wasm_bindgen]
    pub fn create(input: Vec<ItemSpec>, bin: BinSpec) -> Result<AlgorithmenFirstWasm, JsError> {
        let items: Vec<Item> = input.into_iter().map(Into::into).collect();
        let bin: Bin = bin.into();

        let inner = SecondAlgorithmen::create_algorithmen(items, bin)?;

        Ok(Self { inner })
    }

    /// Add items later
    #[wasm_bindgen]
    pub fn add_item(&mut self, input: Vec<ItemSpec>) -> Result<(), JsError> {
        let items: Vec<Item> = input.into_iter().map(Into::into).collect();
        self.inner.add_item(items).map_err(to_js_error)
    }

    /// Remove items
    #[wasm_bindgen]
    pub fn remove_item(&mut self, input: Vec<ItemSpec>) -> Result<(), JsError> {
        let items: Vec<Item> = input.into_iter().map(Into::into).collect();
        self.inner
            .remove_item(items)
            .map_err(|_| JsError::new(&format!("Removed item from algorithmen failed")))
    }

    /// Space left (calls your trait method)
    #[wasm_bindgen]
    pub fn space_left(&self) -> u32 {
        self.inner.space_left()
    }

    /// Run the algorithm.
    /// Currently returns only number of placed items.
    #[wasm_bindgen]
    pub fn calculate(self) -> Result<Vec<CalcResult>, JsError> {
        let sorted = self.inner.calculate().map_err(to_js_error)?;
        let ignore = sorted.removed_items.len();
        let output: Vec<CalcResult> = sorted
            .items
            .into_iter()
            .map(|x| CalcResult::from(x))
            .map(|mut x| {
                x.ignore = ignore;
                x
            })
            .collect();
        Ok(output)
    }
}
