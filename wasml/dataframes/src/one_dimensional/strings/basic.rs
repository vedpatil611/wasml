use super::Strings1d;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Strings1d {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[wasm_bindgen(js_name = shape)]
    pub fn shape_to_js(&self) -> JsValue {
        JsValue::from_serde(self.data.shape()).unwrap()
    }

    pub fn get(&self, index: usize) -> String {
        self.data[index].clone()
    }

    pub fn swap(&mut self, index1: usize, index2: usize) {
        self.data.swap(index1, index2);
    }
}
