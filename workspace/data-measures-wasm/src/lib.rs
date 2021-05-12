use wasm_bindgen::prelude::*;


#[macro_use]
extern crate serde_derive;

extern crate data_measures_core;
use data_measures_core::{MeasureValue, NumericStatistics};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
