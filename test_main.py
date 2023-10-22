import unittest
from main import compute_statistics
import pandas as pd
import time
import psutil



class TestMainFunctions(unittest.TestCase):
    start_time = time.time()

    @classmethod
    def setUpClass(cls):
        cls.df = pd.read_csv("cars.csv", sep=";")

    def test_compute_statistics(self):
        stats = compute_statistics(self.df, "Weight")
        self.assertIn("mean", stats)
        self.assertIn("median", stats)
        self.assertIn("std", stats)
        self.assertIn("size", stats)
        end_time = time.time()
       
        elapsed_time = end_time - start_time

        cpu_percent = psutil.cpu_percent()
        memory_info = psutil.virtual_memory()

        print(f"Elapsed time: {elapsed_time:.4f} seconds")
        print(f"CPU Usage: {cpu_percent}%")
        print(f"Memory Usage: {memory_info.percent}%")



if __name__ == "__main__":
    unittest.main()
