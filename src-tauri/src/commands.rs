use std::{env, fs, path::PathBuf, process::id};

use color_eyre::Result;

#[tauri::command]
pub fn set_wallpaper(path: String) {
  info!("Wallpaper Set {}", path);
  // decode(path).unwrap();
}

fn get_path(id: String) -> Result<PathBuf> {
  //Todo(): Find better way to get application directory!!!
  let mut path = env::current_exe()?;
  path.pop();

  debug!("path is {:?}", &path);
  path.push("wallpapers");
  path.push(id);
  Ok(path)
}

#[tauri::command]
/*
takes the path to the heic file. then decodes and copys its content to a folder next to exe.
1. decodes
2. copys its content to a folder next to exe
3. registers the images and metadata into the database
4. returns a success
*/
pub fn decode_heic(path_to_heic: String, id: String) -> Result<String, String> {
  let wallpaper_export_path = get_path(id).expect("Failed to get wallpaper_export_path");
  fs::create_dir_all(&wallpaper_export_path);
  // todo: Speed this up currently very slow!!!
  // todo: add it to its own thread
  heic_decoder::heic::decode_heic(path_to_heic, wallpaper_export_path.display().to_string())
    .unwrap();

  Ok("".to_string())
}
