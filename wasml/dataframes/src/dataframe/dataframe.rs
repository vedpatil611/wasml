use crate::series::SeriesF64;
use crate::series::SeriesI32;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub enum ColumnType {
    INTEGER,
    FLOAT,
}

#[derive(Serialize, Deserialize)]
pub enum Series {
    Integers(SeriesI32),
    Floats(SeriesF64),
}

#[wasm_bindgen]
pub struct DataFrame {
    data: Vec<Series>,
}

#[wasm_bindgen]
impl DataFrame {
    // ! Expect a array of series
    // #[wasm_bindgen(constructor)]
    // pub fn new(vec_series: JsValue) -> DataFrame {
    //     let series_data: HashMap<String, Series> = serde_wasm_bindgen::from_value(vec_series).unwrap();

    //     DataFrame { data: series_data }
    // }

    #[wasm_bindgen(constructor)]
    pub fn new() -> DataFrame {
        DataFrame {
            data: Vec::new(),
        }
    }

    #[wasm_bindgen(js_name = addColumn)]
    pub fn add_column(&mut self ,datatype: ColumnType, series: JsValue) {
        // let dt: ColumnType = serde_wasm_bindgen::from_value(datatype).unwrap();
        match datatype {
            ColumnType::FLOAT => {
                let ser = serde_wasm_bindgen::from_value(series).unwrap();
                self.data.push(Series::Floats(ser));
            },
            ColumnType::INTEGER => {
                let ser = serde_wasm_bindgen::from_value(series).unwrap();
                self.data.push(Series::Integers(ser));
            }
        }
    }

    pub fn show(&self) -> js_sys::Map {
        let data = js_sys::Map::new();
        self.data.iter().for_each(|ser| {
            // let series_name = serde_wasm_bindgen::to_value((*ser).name).unwrap();
            match &ser {
                Series::Integers(value) => {
                    data.set(&serde_wasm_bindgen::to_value(&value.name()).unwrap(), &value.data());
                },
                Series::Floats(value) => {
                    data.set(&serde_wasm_bindgen::to_value(&value.name()).unwrap(), &value.data());
                }
            }
        });
        data
    }
}
