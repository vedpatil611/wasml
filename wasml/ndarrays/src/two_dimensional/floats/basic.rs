use super::Floats2d;
use crate::one_dimensional::floats::Floats1d;
use ndarray::Array;
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

    /// Append a new row to the array
    pub fn row_append(&mut self, row: &Floats1d) {
        let new_array_vec = self
            .data
            .iter()
            .chain(row.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();
        self.data =
            Array::from_shape_vec((self.row_count() + 1, self.column_count()), new_array_vec)
                .unwrap();
    }

    /// Return the result of appending a new row to the array
    pub fn row_appended(&self, row: &Floats1d) -> Floats2d {
        let new_array_vec = self
            .data
            .iter()
            .chain(row.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        Floats2d {
            data: Array::from_shape_vec((self.row_count() + 1, self.column_count()), new_array_vec)
                .unwrap(),
        }
    }

    /// Append a new column to the array
    pub fn column_append(&mut self, col: &Floats1d) {
        let new_array_vec = self
            .data
            .t()
            .iter()
            .chain(col.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        self.data =
            Array::from_shape_vec((self.row_count(), self.column_count() + 1), new_array_vec)
                .unwrap();
    }

    /// Return the reuslt of appending a new column to the array
    pub fn column_appended(&mut self, col: &Floats1d) -> Floats2d {
        let new_array_vec = self
            .data
            .t()
            .iter()
            .chain(col.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        Floats2d {
            data: Array::from_shape_vec((self.row_count(), self.column_count() + 1), new_array_vec)
                .unwrap(),
        }
    }

    /// Extend the array by appending rows from another array
    pub fn rows_extend(&mut self, other: &Floats2d) {
        let new_array_vec = self
            .data
            .iter()
            .chain(other.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();
        self.data = Array::from_shape_vec(
            (self.row_count() + other.row_count(), self.column_count()),
            new_array_vec,
        )
        .unwrap();
    }

    /// Return the result of extending the array by appending rows
    /// from another array
    pub fn rows_extended(&mut self, other: &Floats2d) -> Floats2d {
        let new_array_vec = self
            .data
            .iter()
            .chain(other.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        Floats2d {
            data: Array::from_shape_vec(
                (self.row_count() + other.row_count(), self.column_count()),
                new_array_vec,
            )
            .unwrap(),
        }
    }

    /// Extend the array by appending columns from another array
    pub fn columns_extend(&mut self, other: &Floats2d) {
        let new_array_vec = self
            .data
            .t()
            .iter()
            .chain(other.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        self.data = Array::from_shape_vec(
            (self.row_count(), self.column_count() + other.column_count()),
            new_array_vec,
        )
        .unwrap();
    }

    /// Return the result of extending the array by appending columns
    /// from another array
    pub fn columns_extended(&mut self, other: &Floats2d) -> Floats2d {
        let new_array_vec = self
            .data
            .t()
            .iter()
            .chain(other.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        Floats2d {
            data: Array::from_shape_vec(
                (self.row_count(), self.column_count() + other.column_count()),
                new_array_vec,
            )
            .unwrap(),
        }
    }
}
