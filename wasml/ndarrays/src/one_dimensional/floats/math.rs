use super::Floats1d;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Floats1d {
    /// Get minimun element
    pub fn min(&self) -> f64 {
        let mut min: f64 = self.data[0];
        self.data.iter().for_each(|elem| {
            if min > *elem {
                min = *elem;
            }
        });
        min
    }

    /// Get maximun element
    pub fn max(&self) -> f64 {
        let mut max: f64 = self.data[0];
        self.data.iter().for_each(|elem| {
            if max < *elem {
                max = *elem;
            }
        });
        max
    }

    /// Get the mean of all the elements in the array
    pub fn mean(&self) -> f64 {
        self.data.mean().unwrap()
    }

    /// Get median off all elements
    pub fn median(&self) -> f64 {
        let mut d = self.data.to_vec();
        d.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = d.len() / 2;
        if d.len() % 2 == 0 {
            (d[mid - 1] + d[mid]) / 2.0
        } else {
            d[mid]
        }
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
