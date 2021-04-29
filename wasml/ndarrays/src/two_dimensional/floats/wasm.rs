use super::Floats2d;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Floats2d {
    /// Create a new Floats2d from javascript
    #[wasm_bindgen(constructor)]
    pub fn new_with_js(js_array: JsValue) -> Floats2d {
        let vector: Vec<Vec<f64>> = js_array.into_serde().unwrap();

        Floats2d::new(vector)
    }

    /// Gives the value contained in the ndarray as a javascript array
    #[wasm_bindgen(getter, js_name = data)]
    pub fn data_to_js(&self) -> JsValue {
        let mut repr: Vec<Vec<f64>> = Vec::new();

        for row_idx in 0..self.data.nrows() {
            repr.push(self.data.row(row_idx).to_vec());
        }

        JsValue::from_serde(&repr).unwrap()
    }

    /// Get the string representation of the underlying ndarray
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{:#?}", self.data)
    }

    /// Clone the object
    #[wasm_bindgen(js_name = clone)]
    pub fn clone_for_wasm(&self) -> Floats2d {
        Floats2d {
            data: self.data.clone(),
        }
    }
}
