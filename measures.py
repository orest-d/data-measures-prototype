class AnalyzerMixin:
    def result(self):
        result = {}
        for measure in self.measures():
            result[measure.identifier] = getattr(self, measure.identifier)
        return result

