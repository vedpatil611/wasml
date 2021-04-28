use ndarrays::one_dimensional::integers::Integers1d;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SeriesI32 {
    name: String,
    data: Integers1d,
    size: usize,
}

#[wasm_bindgen]
impl SeriesI32 {
    #[wasm_bindgen(constructor)]
    pub fn new(name: JsValue, data: JsValue) -> SeriesI32 {
        let col_name = serde_wasm_bindgen::from_value(name).unwrap();
        let serde_data: Vec<i32> = serde_wasm_bindgen::from_value(data).unwrap();
        let col_data = Integers1d::new(serde_data);
        let col_size = col_data.len();
        let new_series = SeriesI32 {
            name: col_name,
            data: col_data,
            size: col_size,
        };

        new_series
    }
    pub fn show(&self) -> String {
        let margin = "#".repeat((self.size * 3) + 6);
        let col_name = self.name.clone();
        let col_size = self.size.to_string();
        let mut c = 0;
        let data: String = self
            .data
            .vector_data()
            .iter()
            .map(|&x| {
                c += 1;
                if c == self.size {
                    x.to_string()
                } else {
                    x.to_string() + ", "
                }
            })
            .collect();
        format!(
            "{}\nName: {}\nData: {}\n(1x{})\n{}",
            margin, col_name, data, col_size, margin
        )
    }

    pub fn data(&self) -> JsValue {
        return self.data.data();
    }

    // pub fn push_data(&mut self, data_item: JsValue) -> js_sys::Int32Array {
    //     let data_item = serde_wasm_bindgen::from_value(data_item).unwrap();
    //     self.data.push(data_item);
    //     self.size += 1;
    //     return js_sys::Int32Array::from(&self.data[..]);
    // }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn update_name(&mut self, column_name: JsValue) -> String {
        let column_name = serde_wasm_bindgen::from_value(column_name).unwrap();
        self.name = column_name;

        self.name.clone()
    }

    pub fn size(&self) -> usize {
        self.size.clone()
    }
}
