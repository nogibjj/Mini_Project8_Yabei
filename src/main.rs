mod lib; // Import the lib module
use std::error::Error;
use std::fs::File;

use csv::ReaderBuilder;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    // 1. Read the cars.csv file
    let file = File::open("cars.csv")?;

    // Create the CSV reader with the specified delimiter
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';') // Set the delimiter to ;
        .has_headers(true)
        .from_reader(file);

    // Find the index of the "Weight" column
    let headers = rdr.headers()?;
    let weight_index = headers
        .iter()
        .position(|h| h == "Weight")
        .ok_or("Weight column not found")?;

    // 2. Extract the "Weight" column from the CSV data
    let mut weights: Vec<f64> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if let Some(weight_str) = record.get(weight_index) {
            if let Ok(weight) = weight_str.parse::<f64>() {
                weights.push(weight);
            }
        }
    }

    // 3. Compute the statistics
    let stats = lib::compute_statistics(&weights);
    println!("Mean: {}", stats.mean);
    println!("Median: {}", stats.median);
    println!("Standard Deviation: {}", stats.std);
    println!("Size: {}", stats.size);
    let end_time = Instant::now();

    // Calculate the elapsed time and resource usage
    let elapsed_time = end_time.duration_since(start_time);
    println!("Total execution time: {:?}", elapsed_time); // Print the elapsed time
                                                          // Memory usage
    let mem_info = sys_info::mem_info().unwrap();
    let memory_usage = mem_info.avail / (1024 * 1024); // Convert KB to MB
    println!("Memory usage: {} MB", memory_usage);

    Ok(())
}
