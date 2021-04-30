use ndarrays::one_dimensional::floats::Floats1d;
use ndarrays::one_dimensional::integers::Integers1d;
pub mod series_f64;
pub mod series_i32;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SeriesF64 {
    name: String,
    data: Floats1d,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SeriesI32 {
    name: String,
    data: Integers1d,
}
