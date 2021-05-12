use super::SeriesI32;
// use crate::dataframe::ColumnType;
// use ndarrays::one_dimensional::integers::Integers1d;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SeriesI32 {
    pub fn sum(&self) -> i32 {
        self.data.sum()
    }

    pub fn product(&self) -> i32 {
        self.data.product()
    }

    pub fn mean(&self) -> i32 {
        self.data.mean()
    }
}
