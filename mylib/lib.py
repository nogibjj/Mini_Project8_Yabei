def compute_statistics(dataframe, column):
    """
    Compute mean, median, standard deviation, and size for a specified column in a DataFrame.
    
    """
    mean = dataframe[column].mean()
    median = dataframe[column].median()
    std = dataframe[column].std()
    size = len(dataframe)
    
    return {'mean': mean, 'median': median, 'std': std, 'size': size}
