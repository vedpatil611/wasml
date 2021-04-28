mod basic;
mod wasm;

use ndarray::{arr1, Array1};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Strings1d {
    data: Array1<String>,
}

// All the rust only functions
impl Strings1d {
    /// Create a new Strings1d
    pub fn new(array: Vec<String>) -> Strings1d {
        Strings1d { data: arr1(&array) }
    }
}
