use std::fs;
use std::path::{Path, PathBuf};
use sysinfo::Disks;

// Scans the system for external disks, filtering out main system partitions
pub fn read_external_disks() -> Result<Vec<PathBuf>, String> {
    let disks = Disks::new_with_refreshed_list();

    println!("Scanning disks...");

    let disks_list: Vec<PathBuf> = disks
        .into_iter()
        .map(|disk| disk.mount_point().to_path_buf())
        .filter(|mount_point| {
            // Cross-platform system directory filtering
            let mp_str = mount_point.to_string_lossy();
            !(mp_str == "/" || mp_str.starts_with("/System") || mp_str.starts_with("C:"))
        })
        .collect();

    if disks_list.is_empty() {
        Err("No external disks found".to_string())
    } else {
        println!("Disks list: {:?}", disks_list);
        Ok(disks_list)
    }
}

// Validates if a given path contains a Rockbox installation
pub fn validate_rockbox_device<P: AsRef<Path>>(mount_point: P) -> Result<PathBuf, String> {
    let mount_ref = mount_point.as_ref();
    let dot_rockbox = mount_ref.join(".rockbox");

    if dot_rockbox.is_dir() {
        println!("Rockbox folder found in: {:?}", mount_ref);
        Ok(mount_ref.to_path_buf())
    } else {
        Err(format!(
            "'{}' is not a Rockbox device (missing .rockbox)",
            mount_ref.display()
        ))
    }
}

// Returns the path to the Playlists directory if it exists
pub fn get_playlists_path<P: AsRef<Path>>(mount_point: P) -> Result<PathBuf, String> {
    let playlist_dir = mount_point.as_ref().join("Playlists");

    if playlist_dir.is_dir() {
        println!("Playlist Directory found at: {:?}", playlist_dir);
        Ok(playlist_dir)
    } else {
        Err("No Playlists/ directory found".to_string())
    }
}

// Lists all valid m3u/m3u8 playlists in a directory, ignoring system hidden files
pub fn get_playlists_list<P: AsRef<Path>>(playlist_dir_path: P) -> Result<Vec<PathBuf>, String> {
    let path_ref = playlist_dir_path.as_ref();

    let entries = fs::read_dir(path_ref).map_err(|e| format!("Error reading directory: {}", e))?;

    let playlists = entries
        .flatten() // Skip results that are Errors
        .filter_map(|entry| {
            let path = entry.path();
            let file_name = path.file_name()?.to_string_lossy();

            // Ignore macOS metadata files and ensure valid extension
            if file_name.starts_with("._") {
                return None;
            }

            let ext = path.extension()?.to_ascii_lowercase();
            if ext == "m3u" || ext == "m3u8" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    Ok(playlists)
}
