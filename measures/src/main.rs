extern crate regex;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::result::Result;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MeasureValue {
    None,
    Text(String),
    Integer(i64),
    Float(f64),
}

impl MeasureValue {
    pub fn none() -> MeasureValue {
        MeasureValue::None
    }
    pub fn text(txt: &str) -> MeasureValue {
        MeasureValue::Text(txt.to_owned())
    }
    pub fn integer(i: i64) -> MeasureValue {
        MeasureValue::Integer(i)
    }
    pub fn float(f: f64) -> MeasureValue {
        MeasureValue::Float(f)
    }
    pub fn float_option(fo: Option<f64>) -> MeasureValue {
        if let Some(f) = fo {
            MeasureValue::Float(f)
        } else {
            MeasureValue::None
        }
    }
    pub fn to_float(&self) -> Option<f64> {
        match &self {
            MeasureValue::None => None,
            MeasureValue::Text(ref s) => s.parse::<f64>().ok(),
            MeasureValue::Integer(ref i) => Some(*i as f64),
            MeasureValue::Float(ref f) => Some(*f),
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            MeasureValue::None => String::from("None"),
            MeasureValue::Text(ref s) => s.to_owned(),
            MeasureValue::Integer(ref i) => format!("{}", *i),
            MeasureValue::Float(ref f) => format!("{}", *f),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Measure {
    identifier: String,
    name: String,
}

impl Measure {
    fn new(identifier: &str) -> Measure {
        Measure {
            identifier: identifier.to_owned(),
            name: identifier.to_owned(),
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct NumericStatistics {
    sum_of_values: f64,
    sum_of_values2: f64,
    sum_of_values3: f64,
    sum_of_values4: f64,
    sum_of_weights: f64,
    minimum: Option<f64>,
    maximum: Option<f64>,
}

impl NumericStatistics {
    fn new() -> NumericStatistics {
        NumericStatistics {
            sum_of_values: 0.0,
            sum_of_values2: 0.0,
            sum_of_values3: 0.0,
            sum_of_values4: 0.0,
            sum_of_weights: 0.0,
            minimum: None,
            maximum: None,
        }
    }

    fn create_empty(&self) -> NumericStatistics {
        NumericStatistics::new()
    }

    fn measures(&self) -> Vec<Measure> {
        vec![
            Measure::new("mean"),
            Measure::new("variance"),
            Measure{identifier:"stddev".to_owned(), name:"standard deviation".to_owned()},
            Measure::new("skewness"),
            Measure::new("kurtosis"),
            Measure::new("minimum"),
            Measure::new("maximum")
        ]
    }

    fn results(&self) -> HashMap<String, MeasureValue> {
        let mut res = HashMap::new();
        res.insert(
            String::from("mean"),
            MeasureValue::float_option(self.mean()),
        );
        res.insert(
            String::from("variance"),
            MeasureValue::float_option(self.variance()),
        );
        res.insert(
            String::from("stddev"),
            MeasureValue::float_option(self.stddev()),
        );
        res.insert(
            String::from("skewness"),
            MeasureValue::float_option(self.skewness()),
        );
        res.insert(
            String::from("kurtosis"),
            MeasureValue::float_option(self.kurtosis()),
        );
        res.insert(
            String::from("minimum"),
            MeasureValue::float_option(self.minimum),
        );
        res.insert(
            String::from("maximum"),
            MeasureValue::float_option(self.maximum),
        );
        res
    }

    fn add_weighted(&mut self, x: &[f64], weight: &[f64]) {
        for (xi, wi) in x.iter().zip(weight.iter()) {
            let wx = wi * xi;
            let wx2 = wx * xi;
            let wx3 = wx2 * xi;
            let wx4 = wx3 * xi;

            self.sum_of_values += wx;
            self.sum_of_values2 += wx2;
            self.sum_of_values3 += wx3;
            self.sum_of_values4 += wx4;
            self.sum_of_weights += wi;
            self.minimum = if let Some(mx) = self.minimum {
                Some(mx.min(*xi))
            } else {
                Some(*xi)
            };
            self.maximum = if let Some(mx) = self.maximum {
                Some(mx.max(*xi))
            } else {
                Some(*xi)
            };
        }
    }

    fn add(&mut self, x: &[f64]) {
        for xi in x.iter() {
            let wx = xi;
            let wx2 = wx * xi;
            let wx3 = wx2 * xi;
            let wx4 = wx3 * xi;

            self.sum_of_values += wx;
            self.sum_of_values2 += wx2;
            self.sum_of_values3 += wx3;
            self.sum_of_values4 += wx4;
            self.sum_of_weights += 1.;
            self.minimum = if let Some(mx) = self.minimum {
                Some(mx.min(*xi))
            } else {
                Some(*xi)
            };
            self.maximum = if let Some(mx) = self.maximum {
                Some(mx.max(*xi))
            } else {
                Some(*xi)
            };
        }
    }

    pub fn add_analyzer(&mut self, analyzer: &Self) {
        self.sum_of_values += analyzer.sum_of_values;
        self.sum_of_values2 += analyzer.sum_of_values2;
        self.sum_of_values3 += analyzer.sum_of_values3;
        self.sum_of_values4 += analyzer.sum_of_values4;
        self.sum_of_weights += analyzer.sum_of_weights;

        if let Some(x) = self.minimum {
            if let Some(y) = analyzer.minimum {
                self.minimum = Some(x.min(y))
            }
        } else {
            self.minimum = analyzer.minimum;
        }

        if let Some(x) = self.maximum {
            if let Some(y) = analyzer.maximum {
                self.maximum = Some(x.max(y))
            }
        } else {
            self.maximum = analyzer.maximum;
        }
    }
    fn mean(&self) -> Option<f64> {
        if self.sum_of_weights == 0.0 {
            None
        } else {
            Some(self.sum_of_values / self.sum_of_weights)
        }
    }

    fn variance(&self) -> Option<f64> {
        if self.sum_of_weights == 0.0 {
            None
        } else {
            let mean = self.sum_of_values / self.sum_of_weights;
            Some(self.sum_of_values2 / self.sum_of_weights - mean * mean)
        }
    }

    fn stddev(&self) -> Option<f64> {
        self.variance().map(|x| x.sqrt())
    }
    /// Skewness: https://en.wikipedia.org/wiki/Skewness
    fn skewness(&self) -> Option<f64> {
        if self.sum_of_weights == 0.0 {
            None
        } else {
            self.stddev().map(|stddev| {
                let mean = self.sum_of_values / self.sum_of_weights;
                let mean_cube = self.sum_of_values3 / self.sum_of_weights;
                (mean_cube - 3.0 * mean * stddev * stddev - mean * mean * mean)
                    / (stddev * stddev * stddev)
            })
        }
    }

    /// Calculates Fisher's kurtosis
    fn kurtosis(&self) -> Option<f64> {
        if self.sum_of_weights == 0.0 {
            None
        } else {
            self.variance().map(|variance| {
                let mean = self.sum_of_values / self.sum_of_weights;
                let mean2 = mean * mean;
                let x2 = self.sum_of_values2 / self.sum_of_weights;
                let x3 = self.sum_of_values3 / self.sum_of_weights;
                let x4 = self.sum_of_values4 / self.sum_of_weights;

                (x4 - 3. * mean2 * mean2 - 4. * mean * x3 + 6. * mean2 * x2) / (variance * variance)
                    - 3.
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut analyzer = NumericStatistics::new();
        let results = analyzer.results();
        for measure in [
            "mean", "variance", "stddev", "skewness", "kurtosis", "minimum", "maximum",
        ]
        .iter()
        {
            assert_eq!(results.get(*measure), Some(&MeasureValue::None));
        }
    }

    #[test]
    fn test_mean() {
        let mut analyzer = NumericStatistics::new();
        analyzer.add_weighted(&[1.0, 2.0, 3.0], &[1.0, 2.0, 1.0]);
        assert_eq!(analyzer.mean(), Some(2.0));
    }

    #[test]
    fn test_mean_add_analyzer() {
        let mut analyzer1 = NumericStatistics::new();
        analyzer1.add_weighted(&[1.0, 2.0, 3.0], &[1.0, 2.0, 1.0]);
        assert_eq!(analyzer1.mean(), Some(2.0));
        let mut analyzer2 = NumericStatistics::new();
        analyzer2.add_weighted(&[3.0, 4.0, 5.0], &[1.0, 2.0, 1.0]);
        assert_eq!(analyzer2.mean(), Some(4.0));
        let mut analyzer = NumericStatistics::new();
        analyzer.add_analyzer(&analyzer1);
        analyzer.add_analyzer(&analyzer2);
        assert_eq!(analyzer.mean(), Some(3.0));
    }

    #[test]
    fn test_all_measures() {
        let mut analyzer = NumericStatistics::new();
        analyzer.add(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(analyzer.mean(), Some(3.0));
        assert_eq!(analyzer.variance(), Some(2.0));
        assert_eq!(analyzer.stddev(), Some(2.0_f64.sqrt()));
        assert!(analyzer.skewness().unwrap().abs()<1e-5);
        assert!((analyzer.kurtosis().unwrap()+1.3)<1e-5);
        assert_eq!(analyzer.minimum, Some(1.0));
        assert_eq!(analyzer.maximum, Some(5.0));
    }
    #[test]
    fn test_all_measures_results() {
        let mut analyzer = NumericStatistics::new();
        analyzer.add(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        let res = analyzer.results();
        assert_eq!(res["mean"].to_float(), Some(3.0));
        assert_eq!(res["variance"].to_float(), Some(2.0));
        assert_eq!(res["stddev"].to_float(), Some(2.0_f64.sqrt()));
        assert!(res["skewness"].to_float().unwrap().abs()<1e-5);
        assert!((res["kurtosis"].to_float().unwrap()+1.3)<1e-5);
        assert_eq!(res["minimum"].to_float(), Some(1.0));
        assert_eq!(res["maximum"].to_float(), Some(5.0));
    }
}

fn main() {
    println!("Hello, world!");
    let mut stat = NumericStatistics::new();
    stat.add(&[1., 2., 3., 4., 5.]);
    println!("Stat:    {:?}", stat);
    println!("Results: {:?}", stat.results());
}
