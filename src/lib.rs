#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use std::process::{Command, Stdio};
extern crate zip_extract;
use std::io::Cursor;
use std::path::PathBuf;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn download_script(url: String) -> Result<String> {
    let res = reqwest::get(&url).await?;
    let script = res.text().await?;
    Ok(script)
}

pub async fn read_script(path: String) -> Result<String> {
    let script = std::fs::read_to_string(path)?;
    Ok(script)
}

pub async fn download_extract_python() -> Result<()> {
    // Check if Python is already installed in the target directory
    let python_path = PathBuf::from("C:\\Users\\Public\\python-3.10.11\\python.exe");
    if python_path.exists() {
        println!("Python installation already exists, skipping download and extraction...");
        return Ok(());
    }

    println!("Downloading and extracting Python embeddable package...");

    // Download the Python embeddable package
    async fn fetch_url(url: String, file_name: String) -> Result<()> {
        let response = reqwest::get(url).await?;
        let mut file = std::fs::File::create(file_name)?;
        let mut content = Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }

    // Download Python package only if not already present
    if !std::path::Path::new("python.zip").exists() {
        fetch_url(
            "https://www.python.org/ftp/python/3.10.11/python-3.10.11-embed-amd64.zip".to_string(),
            "python.zip".to_string(),
        )
        .await
        .unwrap();
    }

    // Extract only if target directory doesn't exist
    let target_dir = PathBuf::from("C:\\Users\\Public\\python-3.10.11");
    if !target_dir.exists() {
        let archive: Vec<u8> = std::fs::read("python.zip").unwrap();
        zip_extract::extract(Cursor::new(archive), &target_dir, true).unwrap();
        println!("Python embeddable package extracted successfully.");
    }

    // Check if pip is already installed by trying to run pip --version
    let pip_check = Command::new(r"C:\Users\Public\python-3.10.11\python.exe")
        .arg("-m")
        .arg("pip")
        .arg("--version")
        .output();

    if let Ok(output) = pip_check {
        if output.status.success() {
            println!("pip is already installed, skipping pip installation...");
            return Ok(());
        }
    }

    // Continue with pip installation if not already present
    if !std::path::Path::new("get-pip.py").exists() {
        fetch_url(
            "https://bootstrap.pypa.io/get-pip.py".to_string(),
            "get-pip.py".to_string(),
        )
        .await
        .unwrap();
    }

    //copy get-pip.py to C:\Users\Public\python-3.10.11
    std::fs::copy("get-pip.py", "C:\\Users\\Public\\python-3.10.11\\get-pip.py").unwrap();

    //execute get-pip.py
    let output = Command::new(r"C:\Users\Public\python-3.10.11\python.exe")
        .arg("C:\\Users\\Public\\python-3.10.11\\get-pip.py")
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");

    let output_string = String::from_utf8(output.stdout).unwrap();

    //edit python installation to allow submodules with the following one-liner: (Get-Content 'C:\Users\Public\python-3.10.11\python310._pth') -replace '#import site','import site' | Set-Content 'C:\Users\Public\python-3.10.11\python310._pth'

    //execute the one-liner to patch pip to allow submodules

    let output = Command::new("powershell.exe")
        .arg("(Get-Content 'C:\\Users\\Public\\python-3.10.11\\python310._pth') -replace '#import site','import site' | Set-Content 'C:\\Users\\Public\\python-3.10.11\\python310._pth'")
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");

    //run C:\\Users\\Public\\python-3.10.11\\python.exe -m pip --version to check if pip is installed

    let output = Command::new(r"C:\Users\Public\python-3.10.11\python.exe")
        .arg("-m")
        .arg("pip")
        .arg("--version")
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");

    let output_string = String::from_utf8(output.stdout).unwrap();

    //check output of pip --version, and if it is installed, print a success message, otherwise print an error message and exit the program

    if output_string.contains("python 3.10") {
        println!("pip installed successfully");
    } else {
        println!("pip failed to install");
        std::process::exit(1);
    }

    Ok(())
}

pub async fn execute_python_script_from_memory(script: &str, imports: &[&str]) -> Result<String> {
    // Logic to install necessary Python packages via pip goes here
    for import in imports {
        let output = Command::new(r"C:\Users\Public\python-3.10.11\python.exe")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg(import)
            .arg("--quiet")  // Add quiet flag to suppress output
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            // Silently continue to next iteration without logging error
            continue;
        }
    }

    println!("Executing Python script...");

    //print script for debugging
    //println!("Script: {}", script);
    //strip quotes from script
    let script = script.trim_matches('\"');

    // Execute the Python script
    let output = Command::new(r"C:\Users\Public\python-3.10.11\python.exe")
        .arg("-c")
        .arg(script)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "Python script exited with error: {}",
            String::from_utf8_lossy(&output.stderr)
        ).into());
    }

    let output_string = String::from_utf8(output.stdout)
        .map_err(|e| e.to_string())?;

    println!("Output from Python script: {}", output_string);
    println!("Python script executed successfully.");

    Ok(output_string)
}

pub fn parse_script_for_imports(script: &str) -> Vec<String> {
    let mut import_statements = Vec::new();
    // Split the script by lines and then by semicolons to handle one-liners
    for part in script.split('\n').flat_map(|line| line.split(';')) {
        if part.contains("import") {
            // Assuming the format is always 'import <module>'
            // This will need adjustment if 'import' can be part of another word or if there are complex import statements
            let import_part = part.split("import").collect::<Vec<&str>>()[1].trim();
            import_statements.push(import_part.to_string());
        }
    }

    import_statements
}