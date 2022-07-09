// use app::wallpaper_tools::decode::decode;

use std::{fmt::format, fs, io::Error, path::PathBuf};

#[tauri::command]
pub fn set_wallpaper(path: String) {
  info!("Wallpaper Set {}", path);
  // decode(path).unwrap();
}

#[tauri::command]
pub fn decode_heic(path_to_heic: String, id: String) -> String {
  // we start by copying moving the heic to a better folder
  let path = PathBuf::from(&path_to_heic);
  let heic_name = path.file_name();
  let data_path = format!("/data/{}", &id);
  fs::create_dir_all(&data_path).unwrap();

  return data_path;
}
