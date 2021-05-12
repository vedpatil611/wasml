use super::SeriesF64;

#[wasm_bindgen]
impl SeriesF64 {
    pub fn mean(&self) -> f64 {
        self.data.mean()
    }

    pub fn max(&self) -> f64 {
        self.data.data.max()
    }

    pub fn min(&self) -> f64 {
        self.data.data.max()
    }

    pub fn median(&self) -> f64 {
        self.data.median()
    }

    #[wasm_bindgen(js_name = standardDeviation)]
    pub fn std_dev(&self) -> f64 {
        self.data.std_dev()
    }
}