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
    pub fn new(array: Vec<String>) -> Strings1d {
        Strings1d { data: arr1(&array) }
    }
}

// All the wasm bindgen functions
#[wasm_bindgen]
impl Strings1d {
    #[wasm_bindgen(constructor)]
    pub fn new_with_js(js_array: JsValue) -> Strings1d {
        let vector: Vec<String> = js_array.into_serde().unwrap();
        Strings1d {
            data: arr1(&vector),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> JsValue {
        JsValue::from_serde(&self.data.to_vec()).unwrap()
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{:#?}", self.data)
    }
}
