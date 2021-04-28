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

    /// Efficiently performs self += alpha * rhs
    pub fn scaled_add(&mut self, other: Integers1d, alpha: i32) {
        self.data.scaled_add(alpha, &other.data);
    }

    /// Get the sum of all the elements in the array
    pub fn sum(&self) -> i32 {
        self.data.sum()
    }

    /// Get the product of all the elements in the array
    pub fn product(&self) -> i32 {
        self.data.product()
    }

    /// Get the mean of all the elements in the array
    pub fn mean(&self) -> i32 {
        self.data.mean().unwrap()
    }
}
