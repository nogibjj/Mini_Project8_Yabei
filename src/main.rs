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
    let start_time = Instant::now();

    let url = "https://github.com/nogibjj/Mini_Project5_Yabei_New/blob/main/cars.csv?raw=true";
    let file_path = "output.txt";

    match extract(url, file_path) {
        Ok(_) => println!(
            "Successfully extracted content from {} to {}",
            url, file_path
        ),
        Err(e) => eprintln!("Error occurred: {}", e),
    }

    let duration = start_time.elapsed();
    println!("Execution time: {:?}", duration);
}
