use super::Integers1d;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Integers1d {
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
