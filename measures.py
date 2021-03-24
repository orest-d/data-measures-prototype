from dataclasses import dataclass
import numpy as np

@dataclass
class Measure:
    identifier: str
    name: str
    def __init__(self, identifier:str, name:str=None):
        self.identifier = identifier
        self.name = name or identifier
                
class AnalyzerMixin:
    def result(self):
        result = {}
        for measure in self.measures():
            result[measure.identifier] = getattr(self, measure.identifier)
        return result
        
class NumericStatistics(AnalyzerMixin):
    def __init__(self):
        self.sum_of_values = 0
        self.sum_of_values2 = 0
        self.sum_of_values3 = 0
        self.sum_of_values4 = 0
        self.sum_of_weights = 0
        self.minimum = None
        self.maximum = None

    def create_empty(self):
        return NumericStatistics()

    def measures(self):
        return [
            Measure("mean"),
            Measure("variance"),
            Measure("stddev", "standard deviation"),
            Measure("skewness"),
            Measure("kurtosis"),
            Measure("minimum"),
            Measure("maximum"),
        ]

    def add(self, x, weight=None, y=None):
        if len(x)==0:
            return self
        if weight is None:
            weight = np.ones(len(x), dtype=np.float64)
        self.sum_of_values += np.sum(weight * x)
        self.sum_of_values2 += np.sum(weight * x ** 2)
        self.sum_of_values3 += np.sum(weight * x ** 3)
        self.sum_of_values4 += np.sum(weight * x ** 4)
        self.sum_of_weights += np.sum(weight)

        xmin = x.min()
        self.minimum = xmin if self.minimum is None else min(xmin, self.minimum)
        xmax = x.max()
        self.maximum = xmax if self.maximum is None else max(xmax, self.maximum)
        return self
    
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

    @property
    def mean(self):
        if self.sum_of_weights == 0:
            return None
        return self.sum_of_values / self.sum_of_weights

    @property
    def variance(self):
        if self.sum_of_weights == 0:
            return None
        mean = self.mean
        return self.sum_of_values2 / self.sum_of_weights - mean ** 2

    @property
    def stddev(self):
        try:
            return np.sqrt(self.variance)
        except:
            return None
    
    @property
    def skewness(self):
        "Skewness: https://en.wikipedia.org/wiki/Skewness"
        if self.sum_of_weights == 0:
            return None
        mean = self.mean
        stddev = self.stddev
        mean_cube = self.sum_of_values3 / self.sum_of_weights
        return (mean_cube - 3 * mean * stddev ** 2 - mean ** 3) / stddev ** 3

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