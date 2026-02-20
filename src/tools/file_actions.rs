use std::fs::File;
use std::io::{self, BufRead};

pub fn open_files_by_path(path: &str) -> Result<File, String> {
    let file = File::open(path).map_err(|_| format!("ERROR: File not found in {}", path))?;

    Ok(file)
}

pub fn read_file(file: File) -> Result<Vec<String>, String> {
    let opened_file: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();

    Ok(opened_file)
}

pub fn write_file(out_path: &str) -> Result<File, String> {
    let write_file =
        File::create(out_path).map_err(|_| format!("ERROR: Unable to write in {}", out_path))?;
    Ok(write_file)
}
