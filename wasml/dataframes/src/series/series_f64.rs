use super::SeriesF64;
use ndarrays::one_dimensional::floats::Floats1d;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SeriesF64 {
    #[wasm_bindgen(constructor)]
    pub fn new(name: JsValue, data: JsValue) -> SeriesF64 {
        let col_name = serde_wasm_bindgen::from_value(name).unwrap();
        let serde_data: Vec<f64> = serde_wasm_bindgen::from_value(data).unwrap();
        let col_data = Floats1d::new(serde_data);

        SeriesF64 {
            name: col_name,
            data: col_data,
        }
    }

    pub fn get_ref(&self) -> JsValue {
        let js_series = self;

        serde_wasm_bindgen::to_value(&js_series).unwrap()
    }

    pub fn show(&self) -> JsValue {
        #[derive(Serialize, Deserialize)]
        struct Display {
            name: String,
            data: Vec<f64>,
        }

        let display_series = Display {
            name: self.name.clone(),
            data: self.data.data.to_vec(),
        };
        serde_wasm_bindgen::to_value(&display_series).unwrap()
    }

    pub fn data(&self) -> JsValue {
        self.data.data_to_js()
    }

    pub fn append(&mut self, data_item: JsValue) {
        let data_item = serde_wasm_bindgen::from_value(data_item).unwrap();
        self.data.append(data_item);
    }

    pub fn appended(&mut self, data_item: JsValue) -> JsValue {
        let data_item = serde_wasm_bindgen::from_value(data_item).unwrap();
        self.data.append(data_item);

        self.data.data_to_js()
    }

    pub fn extend(&mut self, data_arr: JsValue) {
        let data_arr = serde_wasm_bindgen::from_value(data_arr).unwrap();
        let ndarray_data_arr = Floats1d::new(data_arr);
        self.data.extend(ndarray_data_arr)
    }

    pub fn extended(&mut self, data_arr: JsValue) -> JsValue {
        let data_arr = serde_wasm_bindgen::from_value(data_arr).unwrap();
        let ndarray_data_arr = Floats1d::new(data_arr);
        self.data.extend(ndarray_data_arr);

        self.data.data_to_js()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn update_name(&mut self, column_name: JsValue) -> String {
        let column_name = serde_wasm_bindgen::from_value(column_name).unwrap();
        self.name = column_name;

        self.name.clone()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn dtype(&self) -> String {
        String::from("f64")
    }
}
