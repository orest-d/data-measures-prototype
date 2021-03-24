from measures import NumericStatistics
import numpy as np
from scipy.stats import skew, kurtosis
from pytest import approx

class TestNumericStatistics:
    def test_empty(self):
        analyzer = NumericStatistics()
        result = analyzer.result()
        for measure in ["mean", "variance", "stddev", "skewness", "kurtosis"]:
            assert result[measure] is None
    
    def test_mean(self):
        analyzer = NumericStatistics()
        analyzer.add(np.array([1.0,2.0,3.0]),np.array([1.0,2.0,1.0]))
        assert analyzer.mean == 2.0

    def test_mean_add_analyzer(self):
        analyzer1 = NumericStatistics()
        analyzer1.add(np.array([1.0,2.0,3.0]),np.array([1.0,2.0,1.0]))
        assert analyzer1.mean == 2.0
        analyzer2 = NumericStatistics()
        analyzer2.add(np.array([3.0,4.0,5.0]),np.array([1.0,2.0,1.0]))
        assert analyzer2.mean == 4.0
        analyzer = NumericStatistics()
        analyzer.add_analyzer(analyzer1)
        analyzer.add_analyzer(analyzer2)
        assert analyzer.mean == 3.0

    def test_all_measures(self):
        analyzer = NumericStatistics()
        x = np.array([1.0,2.0,3.0,4.0,5.0])
        analyzer.add(x)
        assert analyzer.mean == 3.0
        assert analyzer.mean == approx(x.mean())
        assert analyzer.variance == approx(x.var())
        assert analyzer.stddev == approx(x.std())
        assert analyzer.skewness == approx(skew(x))
        assert analyzer.kurtosis == approx(kurtosis(x))
        assert analyzer.minimum == 1.0
        assert analyzer.maximum == 5.0

