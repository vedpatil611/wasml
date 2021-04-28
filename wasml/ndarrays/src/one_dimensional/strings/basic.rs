use super::Strings1d;
use ndarray::{s, Array1};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Strings1d {
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
    pub fn get(&self, index: usize) -> String {
        self.data[index].clone()
    }

    /// Set the value at the specified index
    pub fn set(&mut self, index: usize, value: String) {
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
    pub fn reversed(&self) -> Strings1d {
        Strings1d {
            data: self.data.slice(s![..;-1]).to_owned(),
        }
    }

    /// Append a new element to the array
    pub fn append(&mut self, element: String) {
        let new = Array1::from_iter(
            self.data
                .clone()
                .into_iter()
                .map(|x| x.clone())
                .chain(std::iter::once(element)),
        );

        self.data = new;
    }

    /// Append a new element to the array and return the result
    pub fn appended(&mut self, element: String) -> Strings1d {
        let new = Array1::from_iter(
            self.data
                .clone()
                .into_iter()
                .map(|x| x.clone())
                .chain(std::iter::once(element)),
        );

        Strings1d { data: new }
    }

    /// Extend an array with another
    pub fn extend(&mut self, other: Strings1d) {
        let new = Array1::from_iter(
            self.data
                .clone()
                .into_iter()
                .map(|x| x.clone())
                .chain(other.data.clone().into_iter().map(|x| x.clone())),
        );

        self.data = new;
    }

    /// Extend an array with another and returns the result
    pub fn extended(&self, other: Strings1d) -> Strings1d {
        let new = Array1::from_iter(
            self.data
                .clone()
                .into_iter()
                .map(|x| x.clone())
                .chain(other.data.clone().into_iter().map(|x| x.clone())),
        );

        Strings1d { data: new }
    }
}
