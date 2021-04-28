use super::Floats1d;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Floats1d {
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
