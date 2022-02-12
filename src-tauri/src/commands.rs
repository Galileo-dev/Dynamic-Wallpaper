#[tauri::command]
pub fn set_wallpaper(path: String) {
  info!("Wallpaper Set {}", path);
}
