use std::fs;
use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;
use std::{collections::HashSet, fs::File};
use sysinfo::Disks;

use crate::tools::file_actions::{open_files_by_path, read_file, write_file};

pub fn get_sorted_lines(path: &str) -> Result<String, String> {
    // Open and read file
    let file = open_files_by_path(path)?;
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
    let file_1 = open_files_by_path(path_1)?;
    let file_2 = open_files_by_path(path_2)?;

    // Read files
    let mut lines_1: Vec<String> = read_file(file_1)?;
    let mut lines_2: Vec<String> = read_file(file_2)?;

    // Merge vectors
    lines_1.append(&mut lines_2);

    // Create final merged file
    let out_path = "./Merged_Playlist.txt";
    let write_file = write_file(out_path)?;

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

pub fn read_external_disk_files() {
    loop {
        // Refresh connected disk list
        let disks = Disks::new_with_refreshed_list();

        println!("Buscando pendrives...");

        for disk in &disks {
            // Get main path for each disk
            let mount_point = disk.mount_point();

            // Set the "ROCKBOX" path identifier
            let dot_rockbox = mount_point.join(".rockbox");

            if dot_rockbox.exists() && dot_rockbox.is_dir() {
                println!("Disk found in: {:?}", mount_point);

                // Directories to read: Playlist/
                let playlist_dir_path = mount_point.join("Playlists");

                if playlist_dir_path.exists() && playlist_dir_path.is_dir() {
                    println!("Files in Directory {:?}:", playlist_dir_path);

                    // Read files
                    match fs::read_dir(&playlist_dir_path) {
                        Ok(entries) => {
                            for entry in entries.flatten() {
                                println!(" - {}", entry.file_name().to_string_lossy());
                            }
                        }
                        Err(e) => println!("Error while reading directory: {}", e),
                    }
                } else {
                    println!(".rockbox found but theres no Playlists/ directory");
                }
            }
        }
        thread::sleep(Duration::from_secs(5));
    }
}
