struct Statistics {
    mean: f64,
    median: f64,
    std: f64,
    size: usize,
}

fn compute_statistics(data: &Vec<f64>) -> Statistics {
    let size = data.len();
    let sum: f64 = data.iter().sum();
    let mean = sum / size as f64;

    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median = if size % 2 == 1 {
        sorted_data[size / 2]
    } else {
        (sorted_data[size / 2 - 1] + sorted_data[size / 2]) / 2.0
    };

    let variance: f64 = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / size as f64;
    let std = variance.sqrt();

    Statistics {
        mean,
        median,
        std,
        size,
    }
}

fn main() {
    let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let stats = compute_statistics(&data);
    println!("Mean: {}", stats.mean);
    println!("Median: {}", stats.median);
    println!("Standard Deviation: {}", stats.std);
    println!("Size: {}", stats.size);
}
