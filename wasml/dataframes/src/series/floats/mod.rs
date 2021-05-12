use ndarrays::one_dimensional::floats::Floats1d;
pub mod series_f64;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SeriesF64 {
    name: String,
    data: Floats1d,
}
