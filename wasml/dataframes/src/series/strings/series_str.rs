use super::SeriesStr;
use crate::dataframe::ColumnType;
use ndarrays::one_dimensional::strings::Strings1d;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SeriesStr {
    #[wasm_bindgen(constructor)]
    pub fn new(name: JsValue, data: JsValue) -> SeriesStr {
        let col_name = serde_wasm_bindgen::from_value(name).unwrap();
        let serde_data: Vec<i32> = serde_wasm_bindgen::from_value(data).unwrap();
        let col_data = Strings1d::new(serde_data);

        SeriesStr {
            name: col_name,
            data: col_data,
        }
    }

        

}