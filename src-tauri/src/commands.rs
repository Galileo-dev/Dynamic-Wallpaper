// use app::wallpaper_tools::decode::decode;

use std::{
  env,
  fmt::format,
  fs,
  io::Error,
  path::{self, Path, PathBuf},
  str::FromStr,
};

use color_eyre::Result;

use tauri::api::path::{data_dir, executable_dir};

#[tauri::command]
pub fn set_wallpaper(path: String) {
  info!("Wallpaper Set {}", path);
  // decode(path).unwrap();
}

#[tauri::command]
/*
takes the path to the heic file. then decodes and copys its content to a folder next to exe.
1. decodes
2. copys its content to a folder next to exe
3. registers the images and metadata into the database
4. returns a success
*/
pub fn decode_heic(path_to_heic: String, id: String) -> Result<(), ()> {
  //! Hard coding path_to_export somthing is broken right know im slowly going insane fixing it
  //! later me will fix this
  // let pathbuf_export = PathBuf::from(
  //   r"E:\Programming\Wallpaper App\Dynamic-Wallpaper\src-tauri\target\debug\wallpapers",
  // );
  // let path_to_export =
  //   r"E:\Programming\Wallpaper App\Dynamic-Wallpaper\src-tauri\target\debug\wallpapers".to_string();

  // let mut pathbuf_export = env::current_dir().unwrap();
  // pathbuf_export.push("/wallpapers");
  // debug!("pathbuf_export = {:?}", &pathbuf_export);
  // let path_to_export = pathbuf_export.into_os_string().into_string().unwrap();

  let mut path_to_export = env::current_exe().unwrap();
  path_to_export.pop();
  println!("path is {:?}", &path_to_export);
  path_to_export.push("wallpapers");
  path_to_export.push(id);
  fs::create_dir_all(&path_to_export).unwrap();
  // todo: Speed this up currently very slow!!!
  // todo: add it to its own thread
  heic_decoder::heic::decode_heic(path_to_heic, path_to_export.display().to_string()).unwrap();

  Ok(())
}
