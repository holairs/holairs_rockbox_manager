use crate::tools::file_actions::{read_file_lines, write_file};
use std::path::Path;

// Merges two playlists into a single file
pub fn merge_playlists<P: AsRef<Path>>(
    path_1: P,
    path_2: P,
    out_path: P,
) -> Result<Vec<String>, String> {
    let mut lines_1 = read_file_lines(&path_1).map_err(|e| e.to_string())?;
    let mut lines_2 = read_file_lines(&path_2).map_err(|e| e.to_string())?;

    lines_1.append(&mut lines_2);

    write_file(&out_path, lines_1.clone()).map_err(|e| e.to_string())?;

    Ok(lines_1)
}

pub fn salute(name: String) -> Result<String, String> {
    if name.trim().is_empty() {
        return Err("No se ingresó un nombre".to_string());
    }
    let res = format!("Hola {}", name);
    Ok(res)
}
