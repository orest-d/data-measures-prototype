use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;
use pyo3::conversion::ToPyObject;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate rand;

#[macro_use]
extern crate serde_derive;

extern crate data_measures_core;
use data_measures_core::{MeasureValue, NumericStatistics};

/*
impl ToPyObject for MeasureValue{
    fn to_object(&self, py: Python) -> PyObject{
        match &self {
            MeasureValue::None => Option::<f64>::None.to_object(py),
            MeasureValue::Text(ref s) => s.to_object(py),
            MeasureValue::Integer(ref i) => i.to_object(py),
            MeasureValue::Float(ref f) => f.to_object(py),
        }

    }
}
*/
fn measure_value_to_object(x: &MeasureValue, py: Python) -> PyObject{
    match &x {
        MeasureValue::None => Option::<f64>::None.to_object(py),
        MeasureValue::Text(ref s) => s.to_object(py),
        MeasureValue::Integer(ref i) => i.to_object(py),
        MeasureValue::Float(ref f) => f.to_object(py),
    }

}

#[pyclass]
struct RSNumericStatistics {
    analyzer: NumericStatistics
}

#[pymethods]
impl RSNumericStatistics {
    #[new]
    pub fn new()->Self{
        RSNumericStatistics{analyzer:NumericStatistics::new()}
    }
    pub fn add(&mut self, x: Vec<f64>) -> PyResult<()>{
        self.analyzer.add(&x);
        Ok(())
    }
    pub fn results(&self, py: Python) -> PyResult<PyObject> {

        //self.analyzer.lock().unwrap().results()
        let d=PyDict::new(py);
        let res = self.analyzer.results();
        
        for (key, val) in res.iter() {
            let val = measure_value_to_object(val, py);
            d.set_item(key, val);
        }
        
        Ok(d.into())
    }

}
/*
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
*/

/// A data-measures module implemented in Rust.
#[pymodule]
fn data_measures_py(py: Python, m: &PyModule) -> PyResult<()> {
//    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<RSNumericStatistics>()?;
    Ok(())
}
