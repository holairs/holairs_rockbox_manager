use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use std::{collections::HashSet, fs::File};

use sysinfo::{DiskKind, Disks};

pub fn read_external_disks() -> Result<Vec<String>, String> {
    let disks = Disks::new_with_refreshed_list();
    let mut disks_list = Vec::new();

    println!("Scanning disks....");

    for disk in &disks {
        let mount_point = disk.mount_point().to_string_lossy();

        if mount_point == "/" || mount_point.starts_with("/System") {
            continue;
        }

        disks_list.push(mount_point.into_owned());
    }

    if disks_list.is_empty() {
        Err("External disks not found".to_string())
    } else {
        println!("Disks list: {:?}", disks_list);
        Ok(disks_list)
    }
}

// Usamos AsRef<Path> para que la función acepte tanto &str como PathBuf
pub fn validate_rockbox_device<P: AsRef<Path>>(mount_point: P) -> Result<String, String> {
    let mount_ref = mount_point.as_ref();

    // Unimos la ruta del punto de montaje con la carpeta oculta .rockbox
    let dot_rockbox = mount_ref.join(".rockbox");

    if dot_rockbox.exists() && dot_rockbox.is_dir() {
        println!("Rockbox folder found in: {:?}", mount_ref);

        // Devolvemos la ruta del punto de montaje como String si es válido
        Ok(mount_ref.to_string_lossy().into_owned())
    } else {
        Err(format!(
            "'{}' is not a Rockbox device (missing .rockbox)",
            mount_ref.display()
        ))
    }
}

pub fn get_playlists_path<P: AsRef<Path>>(mount_point: P) -> Result<PathBuf, String> {
    // Directories to read: Playlist/
    let mount_ref = mount_point.as_ref();
    let playlist_dir_path = mount_ref.join("Playlists");
    if playlist_dir_path.exists() && playlist_dir_path.is_dir() {
        println!("Playlist Directory found in{:?}:", playlist_dir_path);
        Ok(playlist_dir_path)
    } else {
        Err("No Playlists/ Directory found".to_string())
    }
}

pub fn get_playlists_list<P: AsRef<Path>>(playlist_dir_path: P) -> Result<Vec<PathBuf>, String> {
    let path_ref = playlist_dir_path.as_ref();
    let mut playlists = Vec::new();

    let entries = fs::read_dir(path_ref).map_err(|e| format!("Error reading directory: {}", e))?;

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();

        // Filters to get only m3u8 files and no hidden files
        if name_str.starts_with("._") {
            continue;
        }

        // Filters to get only m3u8
        if name_str.ends_with(".m3u8") || name_str.ends_with(".m3u") {
            // Each file path:
            let file_path = path_ref.join(name_str.as_ref());
            playlists.push(file_path);
        }
    }

    // Return list
    Ok(playlists)
}
