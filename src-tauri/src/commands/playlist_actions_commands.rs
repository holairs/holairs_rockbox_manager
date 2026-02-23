use crate::tools::playlist_actions;
use std::path::PathBuf;

#[tauri::command]
pub async fn comm_merge_playlists(
    path_1: PathBuf,
    path_2: PathBuf,
    out_path: PathBuf,
) -> Result<Vec<String>, String> {
    println!("HOLA");
    let res = playlist_actions::merge_playlists(path_1, path_2, out_path);
    println!("{:?}", res);
    res
}

#[tauri::command]
pub async fn comm_salute(name: String) -> Result<String, String> {
    let res = playlist_actions::salute(name);
    println!("{:?}", res);
    res
}
