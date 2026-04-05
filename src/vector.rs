use std::ops::{
    Add,
    Div,
    DivAssign,
    Mul,
    Sub,
};

use num_traits::{
    Num,
    PrimInt,
};
use serde::{
    Deserialize,
    Serialize,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
///
pub struct Vector3<T>
where
    T: Serialize + PartialEq + PartialOrd + Send + Sync + Num + PrimInt + DivAssign + Div + Copy,
{
    ///
    pub x: T,
    ///
    pub y: T,
    ///
    pub z: T,
}
impl<T> Vector3<T>
where
    T: Serialize + PartialEq + PartialOrd + Send + Sync + Num + PrimInt + DivAssign + Div + Copy,
{
    ///
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    ///
    pub fn divide_all(&mut self, divide: T, minus: T) {
        self.x = (self.x.saturating_sub(minus)) / divide;
        self.y = (self.y.saturating_sub(minus)) / divide;
        self.z = (self.z.saturating_sub(minus)) / divide;
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
///
pub struct Vector6<T>
where
    T: Serialize + PartialEq + PartialOrd + PrimInt + DivAssign + Div + Copy,
{
    ///
    pub x: T,
    ///
    pub y: T,
    ///
    pub z: T,
    ///
    pub w: T,
    ///
    pub a: T,
    ///
    pub b: T,
}
impl<T> Vector6<T>
where
    T: Serialize + PartialEq + PartialOrd + Send + Sync + Num + PrimInt + DivAssign + Div + Copy,
{
    ///
    pub const fn new(x: T, y: T, z: T, w: T, a: T, b: T) -> Self {
        Self { x, y, z, w, a, b }
    }
}
impl<T> Add for Vector6<T>
where
    T: Serialize + PartialEq + PartialOrd + Send + Sync + Num + PrimInt + DivAssign + Div + Copy,
{
    type Output = Self;
    ///
    fn add(self, o: Self) -> Self {
        Self::new(
            self.x + o.x,
            self.y + o.y,
            self.z + o.z,
            self.w + o.w,
            self.a + o.a,
            self.b + o.b,
        )
    }
}
impl<T> Sub for Vector6<T>
where
    T: Serialize + PartialEq + PartialOrd + Send + Sync + Num + PrimInt + DivAssign + Div,
{
    type Output = Self;
    ///
    fn sub(self, o: Self) -> Self {
        Self::new(
            self.x - o.x,
            self.y - o.y,
            self.z - o.z,
            self.w - o.w,
            self.a - o.a,
            self.b - o.b,
        )
    }
}
impl<T> Mul for Vector6<T>
where
    T: Serialize + PartialEq + PartialOrd + Send + Sync + Num + PrimInt + DivAssign + Div,
{
    type Output = Self;
    ///
    fn mul(self, o: Self) -> Self {
        Self::new(
            self.x * o.x,
            self.y * o.y,
            self.z * o.z,
            self.w * o.w,
            self.a * o.a,
            self.b * o.b,
        )
    }
}
impl<T> Div for Vector6<T>
where
    T: Serialize + PartialEq + PartialOrd + Send + Sync + Num + PrimInt + DivAssign + Div,
{
    type Output = Self;
    ///
    fn div(self, o: Self) -> Self {
        Self::new(
            self.x / o.x,
            self.y / o.y,
            self.z / o.z,
            self.w / o.w,
            self.a / o.a,
            self.b / o.b,
        )
    }
}
impl<T> Add for Vector3<T>
where
    T: Serialize + PartialEq + PartialOrd + Send + Sync + Num + PrimInt + DivAssign + Div + Copy,
{
    type Output = Self;
    ///
    fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}
impl<T> Sub for Vector3<T>
where
    T: Serialize + PartialEq + PartialOrd + Send + Sync + Num + PrimInt + DivAssign + Div,
{
    type Output = Self;
    ///
    fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}
impl<T> Mul for Vector3<T>
where
    T: Serialize + PartialEq + PartialOrd + Send + Sync + Num + PrimInt + DivAssign + Div,
{
    type Output = Self;
    ///
    fn mul(self, o: Self) -> Self {
        Self::new(self.x * o.x, self.y * o.y, self.z * o.z)
    }
}
impl<T> Div for Vector3<T>
where
    T: Serialize + PartialEq + PartialOrd + Send + Sync + Num + PrimInt + DivAssign + Div,
{
    type Output = Self;
    ///
    fn div(self, o: Self) -> Self {
        Self::new(self.x / o.x, self.y / o.y, self.z / o.z)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
///
pub struct Vector3u32 {
    x: u32,
    y: u32,
    z: u32,
}

#[wasm_bindgen]
impl Vector3u32 {
    #[wasm_bindgen(constructor)]
    ///
    pub fn new(x: u32, y: u32, z: u32) -> Vector3u32 {
        Vector3u32 { x, y, z }
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

    ///
    pub fn divide_all(&mut self, d: u32) {
        self.x /= d;
        self.y /= d;
        self.z /= d;
    }
}

// optional conversions between wasm type and generic type
impl From<Vector3u32> for Vector3<u32> {
    ///
    fn from(v: Vector3u32) -> Self {
        Vector3::new(v.x, v.y, v.z)
    }
}
impl From<Vector3<u32>> for Vector3u32 {
    ///
    fn from(v: Vector3<u32>) -> Self {
        Vector3u32 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
