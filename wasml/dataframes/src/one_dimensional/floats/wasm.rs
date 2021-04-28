use super::Floats1d;
use ndarray::arr1;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Floats1d {
    /// Create a new Floats1d from javascript
    #[wasm_bindgen(constructor)]
    pub fn new_with_js(js_array: JsValue) -> Floats1d {
        let vector: Vec<f64> = js_array.into_serde().unwrap();
        Floats1d {
            data: arr1(&vector),
        }
    }

    /// Gives the value contained in the ndarray as a javascript array
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> JsValue {
        JsValue::from_serde(&self.data.to_vec()).unwrap()
    }

    pub fn vector_data(&self) -> Vec<f64> {
        self.data.to_vec()
    }

    /// Get the string representation of the underlying ndarray
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{:#?}", self.data)
    }
}
