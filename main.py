from mylib.lib import compute_statistics
import pandas as pd

def main():
    # Load the dataset
    df = pd.read_csv("cars.csv")
    
    # For demonstration purposes, let's say we want to compute statistics for a column named 'Price'.
    # Make sure to replace 'Price' with the actual column name from your dataset if it's different.
    column_name = 'Price'
    
    if column_name in df.columns:
        stats = compute_statistics(df, column_name)
        
        # Print the computed statistics
        print(f"Statistics for column '{column_name}':")
        print(f"Mean: {stats['mean']:.2f}")
        print(f"Median: {stats['median']:.2f}")
        print(f"Standard Deviation: {stats['std']:.2f}")
        print(f"Size: {stats['size']}")
    else:
        print(f"Column '{column_name}' not found in the dataset.")

if __name__ == "__main__":
    main()

    
