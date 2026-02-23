use crate::tools::file_actions::{read_file_lines, write_file};
use std::path::{Path, PathBuf};

// Merges two playlists into a single file
pub fn merge_playlists<P: AsRef<Path>>(
    path_1: P,
    path_2: P,
    out_path: P,
) -> Result<String, String> {
    //  Read both files using your new robust function
    let mut lines_1 = read_file_lines(&path_1)?;
    let mut lines_2 = read_file_lines(&path_2)?;

    //  Merge vectors
    lines_1.append(&mut lines_2);

    // Write the unified file using your new write_file function
    write_file(&out_path, lines_1)?;

    Ok("Playlists merged successfully".to_string())
}
