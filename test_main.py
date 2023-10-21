import unittest
from main import compute_statistics
import pandas as pd

class TestMainFunctions(unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        cls.df = pd.read_csv("cars.csv", sep=';')

    def test_compute_statistics(self):
        stats = compute_statistics(self.df, 'Weight')
        self.assertIn('mean', stats)
        self.assertIn('median', stats)
        self.assertIn('std', stats)
        self.assertIn('size', stats)

if __name__ == '__main__':
    unittest.main()
