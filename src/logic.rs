use std::io::{self, BufRead, Write};
use std::{collections::HashSet, fs::File};

pub fn get_sorted_lines(path: &str) -> Result<String, String> {
    // Open and read file
    let file = File::open(path).map_err(|_| format!("ERROR: File not found in {}", path))?;
    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

    // Sort lines
    lines.sort();

    // Trunc file
    let write_file =
        File::create(path).map_err(|_| format!("ERROR: Unable to write in the file {}", path))?;

    let mut writer = io::BufWriter::new(write_file);

    // Add lines to file
    for line in lines {
        writeln!(writer, "{}", line)
            .map_err(|_| format!("ERROR: Failed while writing line: {}", line))?;
    }

    writer
        .flush()
        .map_err(|_| "ERROR: Failed to flush buffer".to_string())?;

    Ok("DONE".to_string())
}

pub fn merge_playlists(path_1: &str, path_2: &str) -> Result<String, String> {
    // Open files
    let file1 = File::open(path_1).map_err(|_| format!("ERROR: File 1 not found in {}", path_1))?;
    let file2 = File::open(path_2).map_err(|_| format!("ERROR: File 2 not found in {}", path_2))?;

    // Read files
    let mut lines_1: Vec<String> = io::BufReader::new(file1)
        .lines()
        .map_while(Result::ok)
        .collect();
    let mut lines_2: Vec<String> = io::BufReader::new(file2)
        .lines()
        .map_while(Result::ok)
        .collect();

    // Merge vectors
    lines_1.append(&mut lines_2);

    // Create final merged file
    let out_path = "./Merged_Playlist.txt";
    let write_file =
        File::create(out_path).map_err(|_| format!("ERROR: Unable to write in {}", out_path))?;

    let mut writer = io::BufWriter::new(write_file);

    // Write each line into unified file
    for line in &lines_1 {
        writeln!(writer, "{}", line)
            .map_err(|_| format!("ERROR: Failed while writing: {}", line))?;
    }

    // Sync buffer with disk
    writer
        .flush()
        .map_err(|_| "ERROR: Failed to flush data".to_string())?;

    Ok("DONE".to_string())
}

pub fn remove_duplicated_lines(mut lines: Vec<String>) -> Result<Vec<String>, String> {
    let mut seen = HashSet::new();

    if lines.is_empty() {
        return Err("The list is empty".to_string());
    }

    lines.retain(|x| seen.insert(x.clone()));
    Ok(lines)
}
