use ndarray::{arr1, Array1};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Integers1d {
    data: Array1<i32>,
}

// All the rust only functions
impl Integers1d {
    /// Create a new Integers1d
    pub fn new(array: Vec<i32>) -> Integers1d {
        Integers1d { data: arr1(&array) }
    }
}

// All the wasm bindgen functions
#[wasm_bindgen]
impl Integers1d {
    // UTITLITY FUNCTIONS

    /// Create a new Integers1d from javascript
    #[wasm_bindgen(constructor)]
    pub fn new_with_js(js_array: JsValue) -> Integers1d {
        let vector: Vec<i32> = js_array.into_serde().unwrap();
        Integers1d {
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

    /// Add two Integers1d's and return the result
    pub fn add(&self, other: &Integers1d) -> Integers1d {
        Integers1d {
            data: &self.data + &other.data,
        }
    }

    /// Substract two Integers1d's and return the result
    pub fn sub(&self, other: &Integers1d) -> Integers1d {
        Integers1d {
            data: &self.data - &other.data,
        }
    }

    /// Multiply two Integers1d's and return the result
    pub fn mul(&self, other: &Integers1d) -> Integers1d {
        Integers1d {
            data: &self.data * &other.data,
        }
    }

    /// Divide two Integers1d's and return the result
    pub fn div(&self, other: &Integers1d) -> Integers1d {
        Integers1d {
            data: &self.data / &other.data,
        }
    }
}
