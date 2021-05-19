use std::collections::HashMap;

use super::ColumnType;
use super::DataFrame;
use super::Series;
use crate::series::floats::SeriesF64;
use crate::series::integers::SeriesI32;
use crate::series::strings::SeriesSTR;
use ndarrays::one_dimensional::floats::Floats1d;
use wasm_bindgen::prelude::*;

// macro_rules! MMM {
//     ($fn_name: ident) => {
//         pub fn $fn_name(&self) -> Floats1d {
//             let mut res: Vec<f64> = Vec::new();
//             self.data.iter().for_each(|ser| match &ser {
//                 Series::Integers(value) => {
//                     res.push(value.$fn_name() as f64);
//                 }
//                 Series::Floats(value) => {
//                     res.push(value.$fn_name());
//                 }
//                 _ => panic!(),
//             });

//             Floats1d::new(res)
//         }
//     };
// }

#[wasm_bindgen]
impl DataFrame {
    #[wasm_bindgen(constructor)]
    pub fn new(vec_series: Vec<JsValue>) -> DataFrame {
        //Get first Series data size

        let mut num_rows = 0;
        let first_series_int: Result<SeriesI32, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(vec_series[0].clone());
        if let Ok(series_int) = first_series_int {
            num_rows = series_int.len()
        }

        let first_series_float: Result<SeriesF64, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(vec_series[0].clone());
        if let Ok(series_float) = first_series_float {
            num_rows = series_float.len()
        }

        let first_series_str: Result<SeriesSTR, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(vec_series[0].clone());
        if let Ok(series_str) = first_series_str {
            num_rows = series_str.len()
        }

        let mut series_data: HashMap<String, Series> = HashMap::new();
        let mut index: Vec<String> = Vec::new();
        let mut num_cols: usize = 0;
        for ser in &vec_series {
            let as_int: Result<SeriesI32, serde_wasm_bindgen::Error> =
                serde_wasm_bindgen::from_value(ser.clone());

            if let Ok(x) = as_int {
                let col_name = x.name();
                if x.len() == num_rows {
                    // series_data[&x.name()] = Series::Integers(x);
                    (*series_data.get_mut(&col_name).unwrap()) = Series::Integers(x);
                    index.push(col_name);
                    num_cols += 1;
                    continue;
                } else {
                    panic!("Series length does not match");
                }
            }

            let as_float: Result<SeriesF64, serde_wasm_bindgen::Error> =
                serde_wasm_bindgen::from_value(ser.clone());

            if let Ok(x) = as_float {
                let col_name = x.name();
                if x.len() == num_rows {
                    // series_data[&x.name()] = Series::Floats(x);
                    (*series_data.get_mut(&col_name).unwrap()) = Series::Floats(x);
                    index.push(col_name);
                    num_cols += 1;
                    continue;
                } else {
                    panic!("Series length does not match");
                }
            }

            let as_str: Result<SeriesSTR, serde_wasm_bindgen::Error> =
                serde_wasm_bindgen::from_value(ser.clone());

            if let Ok(x) = as_str {
                let col_name = x.name();
                if x.len() == num_rows {
                    // series_data[&x.name()] = Series::Strings(x);
                    (*series_data.get_mut(&col_name).unwrap()) = Series::Strings(x);
                    index.push(col_name);
                    num_cols += 1;
                    continue;
                } else {
                    panic!("Series length does not match");
                }
            }
        }

        DataFrame {
            data: series_data,
            index,
            num_rows,
            num_cols,
        }
    }

    // #[wasm_bindgen(js_name = append)]
    // pub fn add_column(&mut self, datatype: ColumnType, series: JsValue) {
    //     match datatype {
    //         ColumnType::FLOAT => {
    //             let ser: SeriesF64 = serde_wasm_bindgen::from_value(series).unwrap();
    //             self.data[&ser.name()] = Series::Floats(ser);
    //         }
    //         ColumnType::INTEGER => {
    //             let ser: SeriesF64 = serde_wasm_bindgen::from_value(series).unwrap();
    //             self.data[&ser.name()] = Series::Floats(ser);
    //         }
    //         ColumnType::STR => {
    //             let ser: SeriesF64 = serde_wasm_bindgen::from_value(series).unwrap();
    //             self.data[&ser.name()] = Series::Floats(ser);
    //         }
    //     }
    // }

    #[wasm_bindgen(js_name = size)]
    pub fn dataframe_size(&self) -> usize {
        self.data.iter().count()
    }

    // pub fn loc(&self, column_name: String) -> String {
    //     let mut res = String::from("");
    //     self.data.iter().for_each(|ser| {
    //         match ser {
    //             Series::Integers(x) => {
    //                 if x.name() == column_name {
    //                     res = x.show();
    //                 }
    //             }
    //             Series::Floats(x) => {
    //                 if x.name() == column_name {
    //                     res = x.show();
    //                 }
    //             }
    //             Series::Strings(x) => {
    //                 if x.name() == column_name {
    //                     res = x.show();
    //                 }
    //             }
    //         };
    //     });

    //     res
    // }

    pub fn ilocr(&self, row: usize) -> js_sys::Array {
        let array = js_sys::Array::new();
        let map = &self.data;
        self.index.iter().for_each(|f| {
            let ser = &map[f];
            match ser {
                Series::Integers(x) => {
                    let val = serde_wasm_bindgen::to_value(&x.get(row)).unwrap();
                    array.push(&val);
                }
                Series::Floats(x) => {
                    let val = serde_wasm_bindgen::to_value(&x.get(row)).unwrap();
                    array.push(&val);
                }
                Series::Strings(x) => {
                    let val = serde_wasm_bindgen::to_value(&x.get(row)).unwrap();
                    array.push(&val);
                }
            };
        });

        // self.data.iter().for_each(|ser| {

        // });

        array
    }

    // pub fn ilocc(&self, col: usize) -> JsValue {
    //     let val: JsValue;
    //     let ser = &self.data[col];
    //     match ser {
    //         Series::Integers(x) => {
    //             val = x.data();
    //         }
    //         Series::Floats(x) => {
    //             val = x.data();
    //         }
    //         Series::Strings(x) => {
    //             val = x.data();
    //         }
    //     };
    //     val
    // }

    #[wasm_bindgen(getter,js_name = display)]
    pub fn show(&self) -> String {
        let mut res: String = String::from("");
        self.index.iter().for_each(|x| {
            let map = &self.data;
            let ser = &map[x];
            match ser {
                Series::Integers(value) => {
                    res.push_str(&value.show());
                }
                Series::Floats(value) => {
                    res.push_str(&value.show());
                }
                Series::Strings(value) => {
                    res.push_str(&value.show());
                }
            }
        });
        // self.data.iter().for_each(|ser| match &ser {
        //     Series::Integers(value) => {
        //         res.push_str(&value.show());
        //     }
        //     Series::Floats(value) => {
        //         res.push_str(&value.show());
        //     }
        //     Series::Strings(value) => {
        //         res.push_str(&value.show());
        //     }
        // });
        res
    }

    // pub fn min(&self, col: JsValue) -> f64 {
    //     let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();
    //     for x in &self.data {
    //         match x {
    //             Series::Floats(value) => {
    //                 if col_name == value.name() {
    //                     return value.min();
    //                 }
    //             }
    //             Series::Integers(value) => {
    //                 if col_name == value.name() {
    //                     return value.min() as f64;
    //                 }
    //             }
    //             Series::Strings(value) => {
    //                 if col_name == value.name() {
    //                     panic!("min function not supported for strings");
    //                 }
    //             }
    //         }
    //     }
    //     panic!("Column name {} not found", col_name);
    // }

    // #[wasm_bindgen(js_name = minColumns)]
    // pub fn min_colunmn(&self) -> Floats1d {
    //     let mut res: Vec<f64> = Vec::new();
    //     self.data.iter().for_each(|ser| match &ser {
    //         Series::Integers(value) => {
    //             res.push(value.min() as f64);
    //         }
    //         Series::Floats(value) => {
    //             res.push(value.min());
    //         }
    //         _ => {}
    //     });

    //     Floats1d::new(res)
    // }

    // pub fn max(&self, col: JsValue) -> f64 {
    //     let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();
    //     for x in &self.data {
    //         match x {
    //             Series::Floats(value) => {
    //                 if col_name == value.name() {
    //                     return value.min();
    //                 }
    //             }
    //             Series::Integers(value) => {
    //                 if col_name == value.name() {
    //                     return value.min() as f64;
    //                 }
    //             }
    //             Series::Strings(value) => if col_name == value.name() {},
    //         }
    //     }
    //     panic!("Column name {} not found", col_name);
    // }

    // #[wasm_bindgen(js_name = maxColumns)]
    // pub fn max_columns(&self) -> Floats1d {
    //     let mut res: Vec<f64> = Vec::new();
    //     self.data.iter().for_each(|ser| match &ser {
    //         Series::Integers(value) => {
    //             res.push(value.max() as f64);
    //         }
    //         Series::Floats(value) => {
    //             res.push(value.max());
    //         }
    //         _ => {}
    //     });

    //     Floats1d::new(res)
    // }

    // pub fn mean(&self, col: JsValue) -> f64 {
    //     let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();
    //     for x in &self.data {
    //         match x {
    //             Series::Floats(value) => {
    //                 if col_name == value.name() {
    //                     return value.mean();
    //                 }
    //             }
    //             Series::Integers(value) => {
    //                 if col_name == value.name() {
    //                     return value.mean();
    //                 }
    //             }
    //             Series::Strings(value) => {
    //                 if col_name == value.name() {
    //                     panic!("mean function not supported for strings");
    //                 }
    //             }
    //         }
    //     }
    //     panic!("Column name {} not found", col_name);
    // }

    // #[wasm_bindgen(js_name = meanColumns)]
    // pub fn mean_columns(&self) -> Floats1d {
    //     let mut res: Vec<f64> = Vec::new();
    //     self.data.iter().for_each(|ser| match &ser {
    //         Series::Integers(value) => {
    //             res.push(value.mean());
    //         }
    //         Series::Floats(value) => {
    //             res.push(value.mean());
    //         }
    //         _ => {}
    //     });

    //     Floats1d::new(res)
    // }

    // pub fn median(&self, col: JsValue) -> f64 {
    //     let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();
    //     for x in &self.data {
    //         match x {
    //             Series::Floats(value) => {
    //                 if col_name == value.name() {
    //                     return value.median();
    //                 }
    //             }
    //             Series::Integers(value) => {
    //                 if col_name == value.name() {
    //                     return value.median();
    //                 }
    //             }
    //             Series::Strings(value) => {
    //                 if col_name == value.name() {
    //                     panic!("median function not supported for strings");
    //                 }
    //             }
    //         }
    //     }
    //     panic!("Column name {} not found", col_name);
    // }

    // #[wasm_bindgen(js_name = medianColumns)]
    // pub fn median_columns(&self) -> Floats1d {
    //     let mut res: Vec<f64> = Vec::new();
    //     self.data.iter().for_each(|ser| match &ser {
    //         Series::Integers(value) => {
    //             res.push(value.median());
    //         }
    //         Series::Floats(value) => {
    //             res.push(value.median());
    //         }
    //         _ => {}
    //     });

    //     Floats1d::new(res)
    // }

    // /// Returns variance of given column with given degree of freedom
    // pub fn variance(&self, col: JsValue, degree_of_freedom: f64) -> f64 {
    //     let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();
    //     for ser in &self.data {
    //         match ser {
    //             Series::Floats(value) => {
    //                 if col_name == value.name() {
    //                     return value.variance(degree_of_freedom);
    //                 }
    //             }
    //             Series::Integers(value) => {
    //                 if col_name == value.name() {
    //                     return value.variance(degree_of_freedom);
    //                 }
    //             }
    //             Series::Strings(_value) => {
    //                 panic!("Variance not supported for String");
    //             }
    //         }
    //     }
    //     panic!("Column name {} not found", col_name);
    // }

    // /// return degree of freedom of all columns
    // pub fn variance_columns(&self, degree_of_freedom: f64) -> Floats1d {
    //     let mut res: Vec<f64> = Vec::new();

    //     for ser in &self.data {
    //         match ser {
    //             Series::Floats(value) => {
    //                 res.push(value.variance(degree_of_freedom));
    //             }
    //             Series::Integers(value) => {
    //                 res.push(value.variance(degree_of_freedom));
    //             }
    //             _ => {}
    //         }
    //     }

    //     Floats1d::new(res)
    // }

    // /// Returns standard deviation of given column with degree of freedom
    // #[wasm_bindgen(js_name = standardDeviation)]
    // pub fn std_dev(&self, col: JsValue, degree_of_freedom: f64) -> f64 {
    //     let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();
    //     for x in &self.data {
    //         match x {
    //             Series::Floats(value) => {
    //                 if col_name == value.name() {
    //                     return value.std_dev(degree_of_freedom);
    //                 }
    //             }
    //             Series::Integers(value) => {
    //                 if col_name == value.name() {
    //                     return value.std_dev(degree_of_freedom);
    //                 }
    //             }
    //             Series::Strings(_value) => {
    //                 panic!("Standard Deviation is not supported for Strings");
    //             }
    //         }
    //     }

    //     panic!("Column name {} not found", col_name);
    // }

    // /// Returns standard deviations of columns
    // #[wasm_bindgen(js_name = standardDeviationsColumns)]
    // pub fn std_dev_columns(&self, degree_of_freedom: f64) -> Floats1d {
    //     let mut res: Vec<f64> = Vec::new();
    //     for x in &self.data {
    //         match x {
    //             Series::Floats(value) => {
    //                 res.push(value.std_dev(degree_of_freedom));
    //             }
    //             Series::Integers(value) => {
    //                 res.push(value.std_dev(degree_of_freedom));
    //             }
    //             _ => {}
    //         }
    //     }

    //     Floats1d::new(res)
    // }
}
