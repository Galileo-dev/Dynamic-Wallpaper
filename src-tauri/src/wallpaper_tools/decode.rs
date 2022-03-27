use libheif_rs::{Channel, ColorSpace, HeifContext, ItemId, Result, RgbChroma};
pub fn decode(path: String) -> Result<()> {
  println!("decoding");
  let ctx = HeifContext::read_from_file(&path)?;
  let handle = ctx.primary_image_handle()?;
  println!("{}", handle.number_of_thumbnails());

  Ok(())
}
