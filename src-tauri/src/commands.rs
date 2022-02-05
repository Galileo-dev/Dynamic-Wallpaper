#[tauri::command]
pub fn set_wallpaper(path: String) {
  println!("Wallpaper Set");
}
