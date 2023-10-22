use reqwest;
use std::fs::File;
use std::io::Write;

fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let mut content = response.bytes()?;
    let mut file = File::create(file_path)?;
    file.write_all(&content)?;
    Ok(())
}

fn main() {
    let url = "https://github.com/nogibjj/Mini_Project5_Yabei_New/blob/main/cars.csv?raw=true";
    let file_path = "output.txt";

    match extract(url, file_path) {
        Ok(_) => println!(
            "Successfully extracted content from {} to {}",
            url, file_path
        ),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}
