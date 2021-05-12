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

    /// Efficiently performs self += alpha * rhs
    pub fn scaled_add(&mut self, alpha: f64, other: Floats1d) {
        self.data.scaled_add(alpha, &other.data);
    }

    /// Get the sum of all the elements in the array
    pub fn sum(&self) -> f64 {
        self.data.sum()
    }

    /// Get the product of all the elements in the array
    pub fn product(&self) -> f64 {
        self.data.product()
    }

    /// Get the mean of all the elements in the array
    pub fn mean(&self) -> f64 {
        self.data.mean().unwrap()
    }

    /// Get the variance of all the elements in the array
    pub fn variance(&self, degree_of_freedom: f64) -> f64 {
        self.data.var(degree_of_freedom)
    }

    /// Get the mean of all the elements in the array
    pub fn standard_deviation(&self, degree_of_freedom: f64) -> f64 {
        self.data.std(degree_of_freedom)
    }
}
