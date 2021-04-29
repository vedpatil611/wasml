use super::Floats2d;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Floats2d {
    /// Add two Floats2d's and return the result
    pub fn add(&self, other: &Floats2d) -> Floats2d {
        Floats2d {
            data: &self.data + &other.data,
        }
    }

    /// Substract two Floats2d's and return the result
    pub fn sub(&self, other: &Floats2d) -> Floats2d {
        Floats2d {
            data: &self.data - &other.data,
        }
    }

    /// Multiply two Floats2d's and return the result
    pub fn mul(&self, other: &Floats2d) -> Floats2d {
        Floats2d {
            data: &self.data * &other.data,
        }
    }

    /// Divide two Floats2d's and return the result
    pub fn div(&self, other: &Floats2d) -> Floats2d {
        Floats2d {
            data: &self.data / &other.data,
        }
    }

    /// Dot product of two arrays
    pub fn dot(&self, other: Floats2d) -> Floats2d {
        Floats2d {
            data: self.data.dot(&other.data),
        }
    }
}
