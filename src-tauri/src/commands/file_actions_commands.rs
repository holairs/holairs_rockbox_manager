use crate::tools::file_actions;
use std::path::PathBuf;

#[tauri::command]
pub async fn comm_read_file_lines(path: PathBuf) -> Result<Vec<String>, String> {
    let res = file_actions::read_file_lines(path);
    println!("{:?}", res);
    res
}
