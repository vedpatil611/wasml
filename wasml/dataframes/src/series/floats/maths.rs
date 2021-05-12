use super::SeriesF64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl SeriesF64 {
    pub fn max(&self) -> f64 {
        let mut max: f64 = 0.0;
        self.data.data.iter().for_each(|elem| {
            if max < *elem {
                max = *elem;
            }
        });
        max
    }

    pub fn min(&self) -> f64 {
        let mut min: f64 = self.data.get(0);
        self.data.data.iter().for_each(|elem| {
            if min > *elem {
                min = *elem;
            }
        });
        min
    }

    pub fn mean(&self) -> f64 {
        self.data.mean()
    }

    // pub fn median(&self) -> f64 {
    //     let mut d = self.data.data.to_vec();
    //     d.sort();
        
    //     let mid = d.len() / 2;
    //     if d.len() % 2 == 0 {
    //         mean()
    //     }

    //     0.0
    // }
    // #[wasm_bindgen(js_name = standardDeviation)]
    // pub fn std_dev(&self) -> f64 {
    //     self.data.standard_deviation()
    // }
}