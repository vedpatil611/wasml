use super::Floats2d;
use ndarray::Axis;
use wasm_bindgen::prelude::*;

use crate::one_dimensional::floats::Floats1d;

#[wasm_bindgen]
impl Floats2d {
    /// Get the var of all the elements in the array
    #[wasm_bindgen(js_name = var)]
    pub fn var(&self, dof: f64) -> f64 {
        self.data.var(dof)
    }

    /// Get the var of each row in the matrix
    #[wasm_bindgen(js_name = varC)]
    pub fn var_c(&self, dof: f64) -> Floats1d {
        Floats1d {
            data: self.data.var_axis(Axis(0), dof),
        }
    }

    /// Get the var of each column in the matrix
    #[wasm_bindgen(js_name = varR)]
    pub fn var_r(&self, dof: f64) -> Floats1d {
        Floats1d {
            data: self.data.var_axis(Axis(1), dof),
        }
    }

    /// Get the mean of all the elements in the array
    #[wasm_bindgen(js_name = std)]
    pub fn std(&self, dof: f64) -> f64 {
        self.data.std(dof)
    }

    /// Get the standard deviation of each row in the matrix
    #[wasm_bindgen(js_name = stdC)]
    pub fn std_c(&self, dof: f64) -> Floats1d {
        Floats1d {
            data: self.data.std_axis(Axis(0), dof),
        }
    }

    /// Get the standard deviation of each column in the matrix
    #[wasm_bindgen(js_name = stdR)]
    pub fn std_r(&self, dof: f64) -> Floats1d {
        Floats1d {
            data: self.data.std_axis(Axis(1), dof),
        }
    }
}
