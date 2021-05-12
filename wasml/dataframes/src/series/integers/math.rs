use super::SeriesI32;
// use crate::dataframe::ColumnType;
// use ndarrays::one_dimensional::integers::Integers1d;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SeriesI32 {
    pub fn sum(&self) -> i32 {
        self.data.sum()
    }

    pub fn product(&self) -> i32 {
        self.data.product()
    }

    pub fn mean(&self) -> f64 {
        let mut c = 0;
        let mut sum = 0;
        let total;
        self.data.data.iter().for_each(|x| {
            c += 1;
            sum += x;
        });

        total = sum as f64 / c as f64;
        total
    }

    pub fn max(&self) -> i32 {
        let mut max: i32 = 0;
        self.data.data.iter().for_each(|elem| {
            if max < *elem {
                max = *elem;
            }
        });
        max
    }

    pub fn min(&self) -> i32 {
        let mut min: i32 = self.data.get(0);
        self.data.data.iter().for_each(|elem| {
            if min > *elem {
                min = *elem;
            }
        });
        min
    }
}
