mod basic;
mod custom_serde;
mod math;
mod wasm;

use ndarray::{arr1, Array1};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Floats1d {
    #[wasm_bindgen(skip)]
    pub data: Array1<f64>,
}

impl Floats1d {
    /// Create a new Floats1d
    pub fn new(array: Vec<f64>) -> Floats1d {
        Floats1d { data: arr1(&array) }
    }
}
