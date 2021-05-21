use super::Floats2d;
use crate::one_dimensional::floats::Floats1d;
use ndarray::{s, Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Floats2d {
    /// Get the number of columns in the array
    pub fn ncols(&self) -> usize {
        self.data.ncols()
    }

    /// Get the number of rows in the array
    pub fn nrows(&self) -> usize {
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
    #[wasm_bindgen(js_name = getCol)]
    pub fn get_col(&self, index: usize) -> Floats1d {
        Floats1d {
            data: self.data.column(index).into_owned(),
        }
    }

    /// Set the column at the specified index
    #[wasm_bindgen(js_name = setCol)]
    pub fn set_col(&mut self, index: usize, array: &Floats1d) {
        self.data
            .column_mut(index)
            .iter_mut()
            .zip(array.data.iter())
            .for_each(|(to, from)| {
                *to = *from;
            });
    }

    /// Get the row at the specified index
    #[wasm_bindgen(js_name = getRow)]
    pub fn get_row(&self, index: usize) -> Floats1d {
        Floats1d {
            data: self.data.row(index).into_owned(),
        }
    }

    /// Set the row at the specified index
    #[wasm_bindgen(js_name = setRow)]
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
    #[wasm_bindgen(js_name = rowAppend)]
    pub fn row_append(&mut self, row: &Floats1d) {
        let new_array_vec = self
            .data
            .iter()
            .chain(row.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();
        self.data = Array::from_shape_vec((self.nrows() + 1, self.ncols()), new_array_vec).unwrap();
    }

    /// Return the result of appending a new row to the array
    #[wasm_bindgen(js_name = rowAppended)]
    pub fn row_appended(&self, row: &Floats1d) -> Floats2d {
        let new_array_vec = self
            .data
            .iter()
            .chain(row.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        Floats2d {
            data: Array::from_shape_vec((self.nrows() + 1, self.ncols()), new_array_vec).unwrap(),
        }
    }

    /// Append a new column to the array
    #[wasm_bindgen(js_name = colAppend)]
    pub fn col_append(&mut self, col: &Floats1d) {
        let new_array_vec = self
            .data
            .t()
            .iter()
            .chain(col.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        self.data = Array::from_shape_vec((self.ncols() + 1, self.nrows()), new_array_vec)
            .unwrap()
            .t()
            .into_owned();
    }

    /// Return the reuslt of appending a new column to the array
    #[wasm_bindgen(js_name = colAppended)]
    pub fn col_appended(&mut self, col: &Floats1d) -> Floats2d {
        let new_array_vec = self
            .data
            .t()
            .iter()
            .chain(col.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        Floats2d {
            data: Array::from_shape_vec((self.ncols() + 1, self.nrows()), new_array_vec)
                .unwrap()
                .t()
                .into_owned(),
        }
    }

    /// Extend the array by appending rows from another array
    #[wasm_bindgen(js_name = rowsExtend)]
    pub fn rows_extend(&mut self, other: &Floats2d) {
        let new_array_vec = self
            .data
            .iter()
            .chain(other.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();
        self.data =
            Array::from_shape_vec((self.nrows() + other.nrows(), self.ncols()), new_array_vec)
                .unwrap();
    }

    /// Return the result of extending the array by appending rows for another
    /// array
    #[wasm_bindgen(js_name = rowsExtended)]
    pub fn rows_extended(&mut self, other: &Floats2d) -> Floats2d {
        let new_array_vec = self
            .data
            .iter()
            .chain(other.data.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        Floats2d {
            data: Array::from_shape_vec(
                (self.nrows() + other.nrows(), self.ncols()),
                new_array_vec,
            )
            .unwrap(),
        }
    }

    /// Extend the array by appending columns from another array
    #[wasm_bindgen(js_name = colsExtend)]
    pub fn cols_extend(&mut self, other: &Floats2d) {
        let new_array_vec = self
            .data
            .t()
            .iter()
            .chain(other.data.t().iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        self.data =
            Array::from_shape_vec((self.ncols() + other.ncols(), self.nrows()), new_array_vec)
                .unwrap()
                .t()
                .into_owned();
    }

    /// Return the result of extending the array by appending columns from
    /// another array
    #[wasm_bindgen(js_name = colsExtended)]
    pub fn cols_extended(&self, other: &Floats2d) -> Floats2d {
        let new_array_vec = self
            .data
            .t()
            .iter()
            .chain(other.data.t().iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        Floats2d {
            data: Array::from_shape_vec(
                (self.ncols() + other.ncols(), self.nrows()),
                new_array_vec,
            )
            .unwrap()
            .t()
            .into_owned(),
        }
    }

    /// Insert a row at the specified index
    #[wasm_bindgen(js_name = rowInsert)]
    pub fn row_insert(&mut self, index: usize, row: Floats1d) {
        let (first, second) = self
            .data
            .multi_slice_mut((s![..index, ..], s![index.., ..]));

        let new_array_vec = first
            .iter()
            .chain(row.data.iter())
            .chain(second.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        self.data = Array::from_shape_vec((self.nrows() + 1, self.ncols()), new_array_vec).unwrap();
    }

    /// Insert a row at the specified index
    #[wasm_bindgen(js_name = rowInserted)]
    pub fn row_inserted(&mut self, index: usize, row: Floats1d) -> Floats2d {
        let (first, second) = self
            .data
            .multi_slice_mut((s![..index, ..], s![index.., ..]));

        let new_array_vec = first
            .iter()
            .chain(row.data.iter())
            .chain(second.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        Floats2d {
            data: Array::from_shape_vec((self.nrows() + 1, self.ncols()), new_array_vec).unwrap(),
        }
    }

    /// Insert a row at the specified index
    #[wasm_bindgen(js_name = colInsert)]
    pub fn col_insert(&mut self, index: usize, row: Floats1d) {
        let (first, second) = self
            .data
            .multi_slice_mut((s![.., ..index], s![.., index..]));

        let new_array_vec = first
            .t()
            .iter()
            .chain(row.data.iter())
            .chain(second.t().iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        self.data = Array::from_shape_vec((self.ncols() + 1, self.nrows()), new_array_vec)
            .unwrap()
            .t()
            .into_owned();
    }

    /// Insert a row at the specified index
    #[wasm_bindgen(js_name = colInserted)]
    pub fn col_inserted(&mut self, index: usize, row: Floats1d) -> Floats2d {
        let (first, second) = self
            .data
            .multi_slice_mut((s![.., ..index], s![.., index..]));

        let new_array_vec = first
            .t()
            .iter()
            .chain(row.data.iter())
            .chain(second.t().iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        Floats2d {
            data: Array::from_shape_vec((self.ncols() + 1, self.nrows()), new_array_vec)
                .unwrap()
                .t()
                .into_owned(),
        }
    }

    /// Remove the row at the specified index and return it
    #[wasm_bindgen(js_name = rowSplice)]
    pub fn row_splice(&mut self, index: usize) -> Floats1d {
        let row = self.get_row(index);
        let (first, second) = self
            .data
            .multi_slice_mut((s![..index, ..], s![(index + 1).., ..]));
        let new_array_vec = first
            .iter()
            .chain(second.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        self.data = Array::from_shape_vec((self.nrows() - 1, self.ncols()), new_array_vec).unwrap();

        row
    }

    /// Remove the row at the specified index and return the modified array and
    /// the removed row
    #[wasm_bindgen(js_name = rowSpliced)]
    pub fn row_spliced(&mut self, index: usize) -> js_sys::Array {
        let row = self.get_row(index);
        let (first, second) = self
            .data
            .multi_slice_mut((s![..index, ..], s![(index + 1).., ..]));
        let new_array_vec = first
            .iter()
            .chain(second.iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        let spliced = Floats2d {
            data: Array::from_shape_vec((self.nrows() - 1, self.ncols()), new_array_vec).unwrap(),
        };

        js_sys::Array::of2(&JsValue::from(spliced), &JsValue::from(row))
    }

    /// Remove the column at the specified index and return it
    #[wasm_bindgen(js_name = colSplice)]
    pub fn col_splice(&mut self, index: usize) -> Floats1d {
        let col = self.get_col(index);
        let (first, second) = self
            .data
            .multi_slice_mut((s![.., ..index], s![.., (index + 1)..]));
        let new_array_vec = first
            .t()
            .iter()
            .chain(second.t().iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        self.data = Array::from_shape_vec((self.nrows(), self.ncols() - 1), new_array_vec)
            .unwrap()
            .t()
            .to_owned();

        col
    }

    /// Remove the column at the specified index and return the modified array
    /// and the removed column
    #[wasm_bindgen(js_name = colSpliced)]
    pub fn column_spliced(&mut self, index: usize) -> js_sys::Array {
        let col = self.get_row(index);
        let (first, second) = self
            .data
            .multi_slice_mut((s![.., ..index], s![.., (index + 1)..]));
        let new_array_vec = first
            .t()
            .iter()
            .chain(second.t().iter())
            .map(|x| *x)
            .collect::<Vec<f64>>();

        let spliced = Floats2d {
            data: Array::from_shape_vec((self.nrows(), self.ncols() - 1), new_array_vec)
                .unwrap()
                .t()
                .into_owned(),
        };

        js_sys::Array::of2(&JsValue::from(spliced), &JsValue::from(col))
    }
}
