// use ndarrays::one_dimensional::floats::Floats1d;
use ndarrays::one_dimensional::integers::Integers1d;
// use ndarrays::one_dimensional::{floats, integers};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct DFSeriesI32 {
    name: String,
    data: Integers1d,
    size: usize,
}

// #[wasm_bindgen]
// #[derive(Serialize, Deserialize)]
// pub struct DFSeriesF64 {
//     name: String,
//     data: Floats1d,
//     size: usize,
// }
// #[derive(Serialize, Deserialize)]
// pub enum Series {
//     I32(DFSeriesI32),
//     F64(DFSeriesF64),
// }

#[wasm_bindgen]
pub struct DataFrame {
    data: Vec<DFSeriesI32>,
}

#[wasm_bindgen]
impl DataFrame {
    #[wasm_bindgen(constructor)]
    pub fn new(vec_series: JsValue) -> DataFrame {
        let series_data: Vec<DFSeriesI32> = serde_wasm_bindgen::from_value(vec_series).unwrap();

        DataFrame { data: series_data }
    }

    pub fn show(&self) -> js_sys::Map {
        let data = js_sys::Map::new();
        self.data.iter().for_each(|ser| {
            let series_name = serde_wasm_bindgen::to_value(&ser.name).unwrap();
            data.set(&series_name, &ser.data.data_to_js());
        });
        data
    }
}
