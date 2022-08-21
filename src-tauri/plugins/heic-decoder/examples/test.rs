use heic_decoder::{heic::decode_heic, loc::get_location, sun::get_sun_pos};
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let path = "D:/Downlaods/image.heic".to_string();
  // decode_heic(path).unwrap();
  let (lat, lon) = get_location().await;
  get_sun_pos(lat, lon);
  Ok(())
}
