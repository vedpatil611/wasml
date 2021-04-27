use super::Strings1d;
use ndarray::arr1;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Strings1d {
    /// Create a new Strings1d from javascript
    #[wasm_bindgen(constructor)]
    pub fn new_with_js(js_array: JsValue) -> Strings1d {
        let vector: Vec<String> = js_array.into_serde().unwrap();
        Strings1d {
            data: arr1(&vector),
        }
    }

    /// Gives the value contained in the ndarray as a javascript array
    #[wasm_bindgen(getter, js_name = data)]
    pub fn data(&self) -> JsValue {
        JsValue::from_serde(&self.data.to_vec()).unwrap()
    }

    /// Get the string representation of the underlying ndarray
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{:#?}", self.data)
    }
}