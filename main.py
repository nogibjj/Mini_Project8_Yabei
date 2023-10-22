from mylib.lib import compute_statistics
import pandas as pd


def main():
    # Load the dataset
    df = pd.read_csv("cars.csv", sep=";")

    column_name = "Weight"

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
