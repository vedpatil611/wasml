use super::ColumnType;
use super::DataFrame;
use super::Series;
use crate::series::floats::SeriesF64;
use crate::series::integers::SeriesI32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl DataFrame {
    #[wasm_bindgen(constructor)]
    pub fn new(vec_series: Vec<JsValue>) -> DataFrame {
        //Get first Series data size
        let mut series_size = 0;
        let first_series_int: Result<SeriesI32, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(vec_series[0].clone());
        if let Ok(series_int) = first_series_int {
            series_size = series_int.len()
        }

        let first_series_float: Result<SeriesF64, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(vec_series[0].clone());
        if let Ok(series_float) = first_series_float {
            series_size = series_float.size()
        }

        let series_data = vec_series
            .iter()
            .map(|series| {
                let as_int: Result<SeriesI32, serde_wasm_bindgen::Error> =
                    serde_wasm_bindgen::from_value(series.clone());

                if let Ok(x) = as_int {
                    if x.len() == series_size {
                        return Series::Integers(x);
                    }
                }

                let as_float: Result<SeriesF64, serde_wasm_bindgen::Error> =
                    serde_wasm_bindgen::from_value(series.clone());
                if let Ok(x) = as_float {
                    if x.size() == series_size {
                        return Series::Floats(x);
                    }
                }

                panic!("Type couldn't be matched");
            })
            .collect();

        DataFrame { data: series_data }
    }

    #[wasm_bindgen(js_name = columns)]
    pub fn show_columns(&self) -> JsValue {
        let mut res: Vec<String> = Vec::new();
        self.data.iter().for_each(|ser| {
            match ser {
                Series::Integers(x) => res.push(x.name()),
                Series::Floats(x) => res.push(x.name()),
            };
        });

        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    #[wasm_bindgen(getter, js_name = dTypes)]
    pub fn show_datatypes(&self) -> JsValue {
        let mut res: Vec<ColumnType> = Vec::new();
        self.data.iter().for_each(|ser| {
            match ser {
                Series::Integers(x) => res.push(x.dtype()),
                Series::Floats(x) => res.push(x.dtype()),
            };
        });

        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    #[wasm_bindgen(js_name = append)]
    pub fn add_column(&mut self, datatype: ColumnType, series: JsValue) {
        // let dt: ColumnType = serde_wasm_bindgen::from_value(datatype).unwrap();
        match datatype {
            ColumnType::FLOAT => {
                let ser = serde_wasm_bindgen::from_value(series).unwrap();
                self.data.push(Series::Floats(ser));
            }
            ColumnType::INTEGER => {
                let ser = serde_wasm_bindgen::from_value(series).unwrap();
                self.data.push(Series::Integers(ser));
            }
        }
    }

    #[wasm_bindgen(js_name = size)]
    pub fn dataframe_size(&self) -> usize {
        self.data.iter().count()
    }

    #[wasm_bindgen(getter,js_name = display)]
    pub fn show(&self) -> js_sys::Map {
        let data = js_sys::Map::new();
        self.data.iter().for_each(|ser| match &ser {
            Series::Integers(value) => {
                data.set(
                    &serde_wasm_bindgen::to_value(&value.name()).unwrap(),
                    &value.data(),
                );
            }
            Series::Floats(value) => {
                data.set(
                    &serde_wasm_bindgen::to_value(&value.name()).unwrap(),
                    &value.data(),
                );
            }
        });
        data
    }
}
