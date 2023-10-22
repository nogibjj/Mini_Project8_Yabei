use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let content = response.bytes()?;
    let mut file = File::create(file_path)?;
    file.write_all(&content)?;
    Ok(())
}

fn main() {
    // Capture memory usage before operation
    let mem_info_before = sys_info::mem_info().unwrap();
    let memory_usage_before = mem_info_before.avail / (1024 * 1024);  // Convert KB to MB
    println!("Memory usage before operation: {} MB", memory_usage_before);

    let url = "https://github.com/nogibjj/Mini_Project5_Yabei_New/blob/main/cars.csv?raw=true";
    let file_path = "output.txt";

    match extract(url, file_path) {
        Ok(_) => println!(
            "Successfully extracted content from {} to {}",
            url, file_path
        ),
        Err(e) => eprintln!("Error occurred: {}", e),
    }

    // Capture memory usage after operation
    let mem_info_after = sys_info::mem_info().unwrap();
    let memory_usage_after = mem_info_after.avail / (1024 * 1024);  // Convert KB to MB
    println!("Memory usage after operation: {} MB", memory_usage_after);
    println!("Memory used by operation: {} MB", memory_usage_before - memory_usage_after);
}
