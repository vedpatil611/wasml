// use super::DataFrame;
// use super::Series;
// use std::collections::HashMap;
// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// impl DataFrame {
//     /// Returns minimun of given column
//     pub fn min(&self, col: JsValue) -> f64 {
//         let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();

//         if self.data.contains_key(&col_name) {
//             match &self.data[&col_name] {
//                 Series::Floats(value) => {
//                     return value.min();
//                 }
//                 Series::Integers(value) => {
//                     return value.min() as f64;
//                 }
//                 Series::Strings(value) => {
//                     panic!("min function not supported for strings");
//                 }
//             }
//         } else {
//             panic!("Column name {} not found", col_name);
//         }
//     }

//     /// Returns minimun of all columns
//     #[wasm_bindgen(js_name = minColumns)]
//     pub fn min_colunmns(&self) -> JsValue {
//         let mut res: HashMap<String, f64> = HashMap::new();

//         for (name, ser) in &self.data {
//             match ser {
//                 Series::Floats(value) => res[name] = value.min(),
//                 Series::Integers(value) => res[name] = value.min() as f64,
//                 _ => {}
//             }
//         }

//         serde_wasm_bindgen::to_value(&res).unwrap()
//     }

//     /// Return maximum element of given column
//     pub fn max(&self, col: JsValue) -> f64 {
//         let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();

//         if self.data.contains_key(&col_name) {
//             match &self.data[&col_name] {
//                 Series::Floats(value) => {
//                     return value.min();
//                 }
//                 Series::Integers(value) => {
//                     return value.min() as f64;
//                 }
//                 Series::Strings(_value) => {
//                     panic!("Max function is nnot supported for Strings");
//                 }
//             }
//         }

//         panic!("Column name {} not found", col_name);
//     }

//     /// Returns maximum elements of all colunms
//     #[wasm_bindgen(js_name = maxColumns)]
//     pub fn max_columns(&self) -> JsValue {
//         let mut res: HashMap<String, f64> = HashMap::new();

//         for (name, ser) in &self.data {
//             match &ser {
//                 Series::Floats(value) => res[name] = value.max(),
//                 Series::Integers(value) => res[name] = value.max() as f64,
//                 _ => {}
//             }
//         }

//         serde_wasm_bindgen::to_value(&res).unwrap()
//     }

//     /// Returns mean of given column
//     pub fn mean(&self, col: JsValue) -> f64 {
//         let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();

//         if self.data.contains_key(&col_name) {
//             match &self.data[&col_name] {
//                 Series::Floats(value) => return value.mean(),
//                 Series::Integers(value) => return value.mean(),
//                 Series::Strings(_value) => {
//                     panic!("Mean function not supported for strings");
//                 }
//             }
//         }

//         panic!("Column name {} not found", col_name);
//     }

//     /// Returns mean of all columns
//     #[wasm_bindgen(js_name = meanColumns)]
//     pub fn mean_columns(&self) -> JsValue {
//         let mut res: HashMap<String, f64> = HashMap::new();

//         for (name, ser) in &self.data {
//             match ser {
//                 Series::Floats(value) => res[name] = value.mean(),
//                 Series::Integers(value) => res[name] = value.mean(),
//                 _ => {}
//             }
//         }

//         serde_wasm_bindgen::to_value(&res).unwrap()
//     }

//     /// Returns median of given column
//     pub fn median(&self, col: JsValue) -> f64 {
//         let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();

//         if self.data.contains_key(&col_name) {
//             match &self.data[&col_name] {
//                 Series::Floats(value) => return value.median(),
//                 Series::Integers(value) => return value.median(),
//                 Series::Strings(_value) => {
//                     panic!("Median function not supported for strings");
//                 }
//             }
//         }

//         panic!("Column name {} not found", col_name);
//     }

//     /// Returns median of all columns
//     #[wasm_bindgen(js_name = medianColumns)]
//     pub fn median_columns(&self) -> JsValue {
//         let mut res: HashMap<String, f64> = HashMap::new();

//         for (name, ser) in &self.data {
//             match &ser {
//                 Series::Floats(value) => res[name] = value.median(),
//                 Series::Integers(value) => res[name] = value.median(),
//                 _ => {}
//             }
//         }

//         serde_wasm_bindgen::to_value(&res).unwrap()
//     }

//     /// Returns variance of given column
//     pub fn variance(&self, col: JsValue, degree_of_freedom: f64) -> f64 {
//         let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();

//         if self.data.contains_key(&col_name) {
//             match &self.data[&col_name] {
//                 Series::Floats(value) => return value.variance(degree_of_freedom),
//                 Series::Integers(value) => return value.variance(degree_of_freedom),
//                 Series::Strings(_value) => {
//                     panic!("Varinance not supported for Strings");
//                 }
//             }
//         }

//         panic!("Column name {} not found", col_name)
//     }

//     /// Returns variance of columns
//     pub fn variance_columns(&self, degree_of_freedom: f64) -> JsValue {
//         let mut res: HashMap<String, f64> = HashMap::new();

//         for (name, ser) in &self.data {
//             match &ser {
//                 Series::Floats(value) => res[name] = value.variance(degree_of_freedom),
//                 Series::Integers(value) => res[name] = value.variance(degree_of_freedom),
//                 _ => {}
//             }
//         }

//         serde_wasm_bindgen::to_value(&res).unwrap()
//     }

//     /// Returns standard deviation of given column
//     #[wasm_bindgen(js_name = standardDeviation)]
//     pub fn std_dev(&self, col: JsValue, degree_of_freedom: f64) -> f64 {
//         let col_name: String = serde_wasm_bindgen::from_value(col).unwrap();

//         if self.data.contains_key(&col_name) {
//             match &self.data[&col_name] {
//                 Series::Floats(value) => return value.std_dev(degree_of_freedom),
//                 Series::Integers(value) => return value.std_dev(degree_of_freedom),
//                 Series::Strings(_value) => {
//                     panic!("Standard deviation not supported for strings");
//                 }
//             }
//         }

//         panic!("Column name {} not found", col_name)
//     }

//     pub fn std_dev_columns(&self, degree_of_freedom: f64) -> JsValue {
//         let mut res: HashMap<String, f64> = HashMap::new();

//         for (name, ser) in &self.data {
//             match &ser {
//                 Series::Floats(value) => res[name] = value.std_dev(degree_of_freedom),
//                 Series::Integers(value) => res[name] = value.std_dev(degree_of_freedom),
//                 _ => {}
//             }
//         }

//         serde_wasm_bindgen::to_value(&res).unwrap()
//     }
// }
