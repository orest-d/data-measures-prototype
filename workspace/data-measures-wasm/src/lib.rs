use wasm_bindgen::prelude::*;


#[macro_use]
extern crate serde_derive;

//extern crate data_measures_core;
use data_measures_core::{MeasureValue, NumericStatistics};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct Analytics {
    analytics: NumericStatistics,
}

#[wasm_bindgen]
impl Analytics {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Analytics {
        Analytics { analytics: NumericStatistics::new() }
    }

    pub fn add(&mut self, x: &[f64]) {
        self.analytics.add(x);
    }

    pub fn results(&self) -> JsValue {
        JsValue::from_serde(&self.analytics.results()).unwrap()
    }

}