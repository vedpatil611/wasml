use crate::series::SeriesF64;
use crate::series::SeriesI32;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod dataframe;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub enum ColumnType {
    INTEGER,
    FLOAT,
}

#[derive(Serialize, Deserialize)]
pub enum Series {
    Integers(SeriesI32),
    Floats(SeriesF64),
}

#[wasm_bindgen]
pub struct DataFrame {
    data: Vec<Series>,
}
