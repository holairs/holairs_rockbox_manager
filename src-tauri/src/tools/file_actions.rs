use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

/// Reads a file and returns its lines as a Vector of Strings
pub fn read_file_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>, String> {
    let file = File::open(&path)
        .map_err(|e| format!("ERROR: Could not open {:?}: {}", path.as_ref(), e))?;

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();

    Ok(lines)
}

/// Overwrites or creates a file with the provided lines
pub fn write_file<P: AsRef<Path>>(path: P, content: Vec<String>) -> Result<(), String> {
    let mut file = File::create(&path)
        .map_err(|e| format!("ERROR: Could not create file {:?}: {}", path.as_ref(), e))?;

    for line in content {
        writeln!(file, "{}", line).map_err(|e| format!("ERROR: Could not write to file: {}", e))?;
    }

    Ok(())
}
