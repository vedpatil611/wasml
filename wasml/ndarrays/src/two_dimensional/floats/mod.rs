pub mod basic;
pub mod math;
pub mod wasm;

use ndarray::Array2;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Floats2d {
    #[wasm_bindgen(skip)]
    pub data: Array2<f64>,
}

impl Floats2d {
    /// Create a new Floats1d
    pub fn new(array: Vec<Vec<f64>>) -> Floats2d {
        let (x, y) = (array.len(), array[0].len());
        Floats2d {
            data: Array2::from_shape_vec((x, y), array.into_iter().flatten().collect()).unwrap(),
        }
    }
}
