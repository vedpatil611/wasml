use super::Integers1d;
use ndarray::{s, Array1};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Integers1d {
    /// Get the length of the array
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get the shape of the array as a javascript array
    #[wasm_bindgen(js_name = shape)]
    pub fn shape_to_js(&self) -> JsValue {
        JsValue::from_serde(self.data.shape()).unwrap()
    }

    /// Get the value at the specified index
    pub fn get(&self, index: usize) -> i32 {
        self.data[index]
    }

    /// Set the value at the specified index
    pub fn set(&mut self, index: usize, value: i32) {
        self.data[index] = value;
    }

    /// Swap the values at the specified indices
    pub fn swap(&mut self, index1: usize, index2: usize) {
        self.data.swap(index1, index2);
    }

    /// Reverse the ndarray
    pub fn reverse(&mut self) {
        self.data = self.data.slice(s![..;-1]).to_owned();
    }

    /// Return the array reversed
    pub fn reversed(&self) -> Integers1d {
        Integers1d {
            data: self.data.slice(s![..;-1]).to_owned(),
        }
    }

    /// Append a new element to the array
    pub fn append(&mut self, element: i32) {
        let new = Array1::from_iter(
            self.data
                .clone()
                .into_iter()
                .map(|x| *x)
                .chain(std::iter::once(element)),
        );

        self.data = new;
    }

    /// Append a new element to the array and return the result
    pub fn appended(&mut self, element: i32) -> Integers1d {
        let new = Array1::from_iter(
            self.data
                .clone()
                .into_iter()
                .map(|x| *x)
                .chain(std::iter::once(element)),
        );

        Integers1d { data: new }
    }

    /// Extend an array with another
    pub fn extend(&mut self, other: Integers1d) {
        let new = Array1::from_iter(
            self.data
                .clone()
                .into_iter()
                .map(|x| *x)
                .chain(other.data.clone().into_iter().map(|x| *x)),
        );

        self.data = new;
    }

    /// Extend an array with another and returns the result
    pub fn extended(&self, other: Integers1d) -> Integers1d {
        let new = Array1::from_iter(
            self.data
                .clone()
                .into_iter()
                .map(|x| x.clone())
                .chain(other.data.clone().into_iter().map(|x| x.clone())),
        );

        Integers1d { data: new }
    }
}
