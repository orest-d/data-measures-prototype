use std::collections::HashMap;

#[derive(Debug)]
pub struct Measure{
    identifier: String,
    name: String
}

impl Measure {
    fn new(identifier:&str) -> Measure {
        Measure{
            identifier: identifier.to_owned(),
            name: identifier.to_owned(),
        }
    }
}

trait AnalyzerMixin{
    fn numeric_results(&self) -> HashMap<String,f64>;
    /*
    {
        let mut result = HashMap::new();
        for measure in self.measures(){
            result[measure.identifier] = getattr(self, measure.identifier)
        }
        result
    }
    */
}

struct NumericStatistics{
    sum_of_values: f64,
    sum_of_values2: f64,
    sum_of_values3: f64,
    sum_of_values4: f64,
    sum_of_weights: f64,
    minimum: Option<f64>,
    maximum: Option<f64>
}


impl NumericStatistics{
    fn new() -> NumericStatistics{
        NumericStatistics {
            sum_of_values: 0.0,
            sum_of_values2: 0.0,
            sum_of_values3: 0.0,
            sum_of_values4: 0.0,
            sum_of_weights: 0.0,
            minimum: None,
            maximum: None
        }
    }

    fn create_empty(&self) -> NumericStatistics{
        NumericStatistics::new()
    }

    fn measures(&self) -> Vec<Measure>{
        vec![
            Measure::new("mean"),
/*            Measure("variance"),
            Measure("stddev", "standard deviation"),
            Measure("skewness"),
            Measure("kurtosis"),
            Measure("minimum"),
            Measure("maximum"),*/
        ]
    }

    fn add_weighted(&mut self, x:&[f64], weight:&[f64]){
        for (xi, wi) in x.iter().zip(weight.iter()){
            let wx = wi*xi;
            let wx2 = wx*xi;
            let wx3 = wx2*xi;
            let wx4 = wx3*xi;

            self.sum_of_values += wx;
            self.sum_of_values2 += wx2;
            self.sum_of_values3 += wx3;
            self.sum_of_values4 += wx4;
            self.sum_of_weights += wi;
            self.minimum = if let Some(mx) = self.minimum {Some(mx.min(*xi))} else {Some(*xi)};
            self.maximum = if let Some(mx) = self.maximum {Some(mx.max(*xi))} else {Some(*xi)};
        }
    }
/*        
    def add_analyzer(self, analyzer):
        assert isinstance(analyzer, NumericStatistics)
        self.sum_of_values += analyzer.sum_of_values
        self.sum_of_values2 += analyzer.sum_of_values2
        self.sum_of_values3 += analyzer.sum_of_values3
        self.sum_of_values4 += analyzer.sum_of_values4
        self.sum_of_weights += analyzer.sum_of_weights

        if self.minimum is None:
            self.minimum = analyzer.minimum
        elif analyzer.minimum is not None:
            self.minimum = min(self.minimum, analyzer.minimum)

        if self.maximum is None:
            self.maximum = analyzer.maximum
        elif analyzer.maximum is not None:
            self.maximum = max(self.maximum, analyzer.maximum)

        return self
*/
    fn mean(&self) -> Option<f64>{
        if self.sum_of_weights == 0.0 {
            None
        }
        else{
            Some(self.sum_of_values / self.sum_of_weights)
        }
    }

    fn variance(&self) -> Option<f64>{
        if self.sum_of_weights == 0.0{
            None
        }
        else{
            let mean = self.sum_of_values / self.sum_of_weights;
            Some(self.sum_of_values2 / self.sum_of_weights - mean * mean)
        }
    }

    fn stddev(&self) -> Option<f64>{
        self.variance().map(|x| x.sqrt())
    }
    
    /// Skewness: https://en.wikipedia.org/wiki/Skewness
    fn skewness(&self) -> Option<f64>{
        if self.sum_of_weights == 0.0 {
            None
        }
        else{
            let mean = self.sum_of_values / self.sum_of_weights;
            let stddev = self.stddev().unwrap();
            let mean_cube = self.sum_of_values3 / self.sum_of_weights;
            Some((mean_cube - 3.0 * mean * stddev * stddev - mean * mean * mean) / (stddev * stddev * stddev))
        }
    }
/*

    @property
    def kurtosis(self):
        "Calculates Fisher's kurtosis"
        if self.sum_of_weights == 0:
            return None
        mean = self.mean
        stddev = self.stddev
        x2 = self.sum_of_values2 / self.sum_of_weights
        x3 = self.sum_of_values3 / self.sum_of_weights
        x4 = self.sum_of_values4 / self.sum_of_weights

        try:
            return (x4 - 3 * mean ** 4  - 4 * mean * x3 + 6 * mean ** 2 * x2) / stddev  ** 4 - 3
        except:
            return None
*/
}

fn main() {
    println!("Hello, world!");
}
