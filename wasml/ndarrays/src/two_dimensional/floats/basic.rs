use super::Floats2d;
use crate::one_dimensional::floats::Floats1d;
use ndarray::s;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Floats2d {
    /// Get the number of columns in the array
    pub fn column_count(&self) -> usize {
        self.data.ncols()
    }

    /// Get the number of rows in the array
    pub fn row_count(&self) -> usize {
        self.data.nrows()
    }

    /// Get the shape of array as a javascript
    #[wasm_bindgen(js_name = shape)]
    pub fn shape_to_js(&self) -> JsValue {
        JsValue::from_serde(self.data.shape()).unwrap()
    }

    /// Get the value at the given index
    pub fn get(&self, index: js_sys::Array) -> f64 {
        let index = (
            index.get(0).as_f64().unwrap() as usize,
            index.get(1).as_f64().unwrap() as usize,
        );
        self.data[index]
    }

    /// Set the value at the specified index
    pub fn set(&mut self, index: js_sys::Array, value: f64) {
        let index = (
            index.get(0).as_f64().unwrap() as usize,
            index.get(1).as_f64().unwrap() as usize,
        );
        self.data[index] = value;
    }

    /// Swap the values at the specified indices
    pub fn swap(&mut self, index1: js_sys::Array, index2: js_sys::Array) {
        let index1 = (
            index1.get(0).as_f64().unwrap() as usize,
            index1.get(1).as_f64().unwrap() as usize,
        );
        let index2 = (
            index2.get(0).as_f64().unwrap() as usize,
            index2.get(1).as_f64().unwrap() as usize,
        );

        self.data.swap(index1, index2);
    }

    /// Get the column at the specified index
    pub fn get_column(&self, index: usize) -> Floats1d {
        Floats1d {
            data: self.data.column(index).into_owned(),
        }
    }

    /// Set the column at the specified index
    pub fn set_column(&mut self, index: usize, array: &Floats1d) {
        self.data
            .column_mut(index)
            .iter_mut()
            .zip(array.data.iter())
            .for_each(|(to, from)| {
                *to = *from;
            });
    }

    /// Get the row at the specified index
    pub fn get_row(&self, index: usize) -> Floats1d {
        Floats1d {
            data: self.data.row(index).into_owned(),
        }
    }

    /// Set the row at the specified index
    pub fn set_row(&mut self, index: usize, array: &Floats1d) {
        self.data
            .row_mut(index)
            .iter_mut()
            .zip(array.data.iter())
            .for_each(|(to, from)| {
                *to = *from;
            });
    }
}
