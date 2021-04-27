use ndarray::{arr1, Array1};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Floats1d {
    data: Array1<f64>,
}

// All the rust only functions
impl Floats1d {
    /// Create a new Floats1d
    pub fn new(array: Vec<f64>) -> Floats1d {
        Floats1d { data: arr1(&array) }
    }
}

// All the wasm bindgen functions
#[wasm_bindgen]
impl Floats1d {
    // UTITLITY FUNCTIONS

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

    /// Get the string representation of the underlying ndarray
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{:#?}", self.data)
    }

    // BASIC MATH OPERATIONS

    /// Add two Floats1d's and return the result
    pub fn add(&self, other: &Floats1d) -> Floats1d {
        Floats1d {
            data: &self.data + &other.data,
        }
    }

    /// Substract two Floats1d's and return the result
    pub fn sub(&self, other: &Floats1d) -> Floats1d {
        Floats1d {
            data: &self.data - &other.data,
        }
    }

    /// Multiply two Floats1d's and return the result
    pub fn mul(&self, other: &Floats1d) -> Floats1d {
        Floats1d {
            data: &self.data * &other.data,
        }
    }

    /// Divide two Floats1d's and return the result
    pub fn div(&self, other: &Floats1d) -> Floats1d {
        Floats1d {
            data: &self.data / &other.data,
        }
    }
}
