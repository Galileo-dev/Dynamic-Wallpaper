use app::wallpaper_tools::decode::decode;

#[tauri::command]
pub fn set_wallpaper(path: String) {
  info!("Wallpaper Set {}", path);
  decode(path).unwrap();
}
