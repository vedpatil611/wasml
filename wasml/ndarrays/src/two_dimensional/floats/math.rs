use crate::one_dimensional::floats::Floats1d;

use super::Floats2d;
use ndarray::Axis;
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

    /// Dot product of two matrices
    pub fn dot(&self, other: Floats2d) -> Floats2d {
        Floats2d {
            data: self.data.dot(&other.data),
        }
    }

    /// Efficiently performs self += alpha * rhs
    pub fn scaled_add(&mut self, alpha: f64, other: Floats2d) {
        self.data.scaled_add(alpha, &other.data);
    }

    /// Get the sum of all the elements in the matrix
    pub fn sum(&self) -> f64 {
        self.data.sum()
    }

    /// Get the sum of each row in the matrix
    pub fn column_sum(&self) -> Floats1d {
        Floats1d {
            data: self.data.sum_axis(Axis(0)),
        }
    }

    /// Get the sum of each column in the matrix
    pub fn row_sum(&self) -> Floats1d {
        Floats1d {
            data: self.data.sum_axis(Axis(1)),
        }
    }

    /// Get the product of all the elements in the array
    pub fn product(&self) -> f64 {
        self.data.product()
    }

    /// Get the mean of all the elements in the array
    pub fn mean(&self) -> f64 {
        self.data.mean().unwrap()
    }

    /// Get the mean of each row in the matrix
    pub fn column_mean(&self) -> Floats1d {
        Floats1d {
            data: self.data.mean_axis(Axis(0)).unwrap(),
        }
    }

    /// Get the mean of each column in the matrix
    pub fn row_mean(&self) -> Floats1d {
        Floats1d {
            data: self.data.mean_axis(Axis(1)).unwrap(),
        }
    }

    /// Get the variance of all the elements in the array
    pub fn variance(&self, degree_of_freedom: f64) -> f64 {
        self.data.var(degree_of_freedom)
    }

    /// Get the variance of each row in the matrix
    pub fn column_variance(&self, degree_of_freedom: f64) -> Floats1d {
        Floats1d {
            data: self.data.var_axis(Axis(0), degree_of_freedom),
        }
    }

    /// Get the variance of each column in the matrix
    pub fn row_variance(&self, degree_of_freedom: f64) -> Floats1d {
        Floats1d {
            data: self.data.var_axis(Axis(1), degree_of_freedom),
        }
    }

    /// Get the mean of all the elements in the array
    pub fn standard_deviation(&self, degree_of_freedom: f64) -> f64 {
        self.data.std(degree_of_freedom)
    }

    /// Get the standard deviation of each row in the matrix
    pub fn column_standard_deviation(&self, degree_of_freedom: f64) -> Floats1d {
        Floats1d {
            data: self.data.std_axis(Axis(0), degree_of_freedom),
        }
    }

    /// Get the standard deviation of each column in the matrix
    pub fn row_standard_deviation(&self, degree_of_freedom: f64) -> Floats1d {
        Floats1d {
            data: self.data.std_axis(Axis(1), degree_of_freedom),
        }
    }
}
