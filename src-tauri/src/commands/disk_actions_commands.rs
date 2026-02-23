use crate::tools::disk_actions; // Importas tu lógica pura
use std::path::PathBuf;

#[tauri::command]
pub async fn get_external_disks() -> Result<Vec<PathBuf>, String> {
    disk_actions::read_external_disks()
}

#[tauri::command]
pub async fn check_rockbox(path: String) -> Result<PathBuf, String> {
    disk_actions::validate_rockbox_device(path)
}
