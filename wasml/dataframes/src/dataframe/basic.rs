use super::ColumnType;
use super::DataFrame;
use super::Series;
use crate::series::floats::SeriesF64;
use crate::series::integers::SeriesI32;
use crate::series::strings::SeriesSTR;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl DataFrame {
    #[wasm_bindgen(js_name = columns)]
    pub fn show_columns(&self) -> JsValue {
        let mut res: Vec<String> = self.index.clone();

        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    // #[wasm_bindgen(js_name = dTypes)]
    // pub fn show_datatypes(&self) -> JsValue {
    //     let mut res: HashMap<String, ColumnType> = HashMap::new();

    //     for name in &self.index {
    //         match &self.data[name] {
    //             Series::Floats(_)   => res[name] = ColumnType::FLOAT,
    //             Series::Integers(_) => res[name] = ColumnType::INTEGER,
    //             Series::Strings(_)  => res[name] = ColumnType::STR,
    //         }
    //     }

    //     serde_wasm_bindgen::to_value(&res).unwrap()
    // }
}
