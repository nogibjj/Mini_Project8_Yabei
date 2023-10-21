use std::fs::File;
use std::io::Write;
use reqwest;

fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let mut content = response.bytes()?;
    let mut file = File::create(file_path)?;
    file.write_all(&content)?;
    Ok(())
}
