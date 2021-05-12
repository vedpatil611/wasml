use super::SeriesSTR;
use ndarrays::one_dimensional::strings::Strings1d;
// use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SeriesSTR {
    #[wasm_bindgen(constructor)]
    pub fn new(name: JsValue, data: JsValue) -> SeriesSTR {
        let col_name = serde_wasm_bindgen::from_value(name).unwrap();
        let serde_data: Vec<String> = serde_wasm_bindgen::from_value(data).unwrap();
        let col_data = Strings1d::new(serde_data);

        SeriesSTR {
            name: col_name,
            data: col_data,
        }
    }

    #[wasm_bindgen(getter,js_name = display)]
    pub fn show(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }

    pub fn append(&mut self, element: String) {
        self.data.append(element);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
