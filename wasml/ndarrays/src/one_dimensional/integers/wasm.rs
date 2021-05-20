use super::Integers1d;
use ndarray::Array1;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Integers1d {
    /// Create a new Integers1d from javascript
    #[wasm_bindgen(constructor)]
    pub fn new_with_js(value: JsValue) -> Integers1d {
        serde_wasm_bindgen::from_value(value).unwrap()
    }

    /// Create a new Integers1d of the given length filled with zeros
    #[wasm_bindgen(js_name = "newWithZeros")]
    pub fn new_with_zeros(len: usize) -> Integers1d {
        Integers1d {
            data: Array1::zeros([len]),
        }
    }

    /// Create a new Integers1d of the given length filled with ones
    #[wasm_bindgen(js_name = "newWithOnes")]
    pub fn new_with_ones(len: usize) -> Integers1d {
        Integers1d {
            data: Array1::ones([len]),
        }
    }

    /// Create a new Integers1d of the given length filled with the specified
    /// element
    #[wasm_bindgen(js_name = "newWithElement")]
    pub fn new_with_elem(len: usize, element: i32) -> Integers1d {
        Integers1d {
            data: Array1::from_elem([len], element),
        }
    }

    /// Create a new Integers1d of the length calling the specified function
    /// without any arguments
    #[wasm_bindgen(js_name = "newWithSimpleFunc")]
    pub fn new_with_simple_func(len: usize, js_func: js_sys::Function) -> Integers1d {
        Integers1d {
            data: Array1::from_shape_simple_fn([len], move || {
                js_func.call0(&JsValue::NULL).unwrap().as_f64().unwrap() as i32
            }),
        }
    }

    /// Create a new Integers1d of the length calling the specified function
    /// with the index as the argument
    #[wasm_bindgen(js_name = "newWithFunc")]
    pub fn new_with_func(len: usize, js_func: js_sys::Function) -> Integers1d {
        Integers1d {
            data: Array1::from_shape_fn([len], move |idx| {
                js_func
                    .call1(&JsValue::NULL, &JsValue::from(idx as u32))
                    .unwrap()
                    .as_f64()
                    .unwrap() as i32
            }),
        }
    }

    /// Gives the JSON representation of the array
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).unwrap()
    }

    /// Gives the value contained in the ndarray as a javascript array
    #[wasm_bindgen(getter, js_name = data)]
    pub fn data_to_js(&self) -> JsValue {
        self.to_json()
    }

    /// Get the string representation of the underlying ndarray
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{:#?}", self.data)
    }

    /// Clone the object
    #[wasm_bindgen(js_name = clone)]
    pub fn clone_for_wasm(&self) -> Integers1d {
        Integers1d {
            data: self.data.clone(),
        }
    }
}
