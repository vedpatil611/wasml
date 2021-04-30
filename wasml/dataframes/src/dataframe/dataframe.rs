use crate::series::SeriesF64;
use crate::series::SeriesI32;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub enum Series {
    Integers(SeriesI32),
    Floats(SeriesF64),
}

#[wasm_bindgen]
pub struct DataFrame {
    #[wasm_bindgen(skip)]
    pub data: HashMap<String, Series>,
}

#[wasm_bindgen]
impl DataFrame {
    // ! Expect a array of series
    #[wasm_bindgen(constructor)]
    pub fn new(vec_series: JsValue) -> DataFrame {
        let series_data: HashMap<String, Series> = serde_wasm_bindgen::from_value(vec_series).unwrap();

        DataFrame { data: series_data }
    }

    // pub fn show(&self) -> js_sys::Map {
    //     let data = js_sys::Map::new();
    //     self.data.iter().for_each(|ser| {
    //         let series_name = serde_wasm_bindgen::to_value(&ser.name).unwrap();
    //         data.set(&series_name, &ser.data.data_to_js());
    //     });
    //     data
    // }
}
