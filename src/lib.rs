pub mod heic;
pub mod list;
pub mod loc;
pub mod sun;
pub mod xml;

#[cfg(test)]
mod tests {
  use crate::heic::decode_heic;
  use color_eyre::eyre::Result;
  use std::{fs, path::PathBuf};

  #[test]
  fn decode_heic_image() -> Result<()> {
    let image_path = PathBuf::from("./test/image.heic");
    assert!(image_path.exists());
    let export_path = PathBuf::from("./test/export/test01");
    fs::create_dir_all(&export_path)?;
    assert!(export_path.exists());
    decode_heic(image_path, export_path)?;
    Ok(())
  }
}
