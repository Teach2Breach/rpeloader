#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

extern crate zip_extract;
use std::io::{self, Cursor};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use rpeloader::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup environment.
    rpeloader::download_extract_python().await?;
    
    // Ask user for a Python script path.
    println!("Please enter the path to your Python script: ");
    let mut script_path = String::new();
    io::stdin().read_line(&mut script_path)?;

    // Trim the input string.
    script_path = script_path.trim().to_string();

    // Check if the script_path is a URL starting with http:// or https://
    let script = if script_path.starts_with("http://") || script_path.starts_with("https://") {
        // If it is a URL, attempt to download the script
        download_script(script_path).await?
    } else if !script_path.starts_with("http://") && !script_path.starts_with("https://") && script_path.ends_with(".py") {
        // If it's not a URL but ends with .py, assume it's a local Python script file and read it
        read_script(script_path).await?
    } else {
        // If it's neither, treat the script_path as a Python one-liner or a path to a local script and convert it to a String
        format!("\"{}\"", script_path.replace("\"", "\\\""))
    };

    let imports: Vec<String> = rpeloader::parse_script_for_imports(&script);
    let imports_str: Vec<&str> = imports.iter().map(|s| s.as_str()).collect();
    
    println!("imports: {:?}", imports_str);
    
    // Execute the Python script.
    execute_python_script_from_memory(&script, &imports_str).await?;

    Ok(())
}
