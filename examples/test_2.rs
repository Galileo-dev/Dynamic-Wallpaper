use app::{heic::decode_heic, sun::get_sun_pos};
use std::io::Result;
fn main() -> Result<()> {
    let path = "C:/Users/fionn/Documents/anime room (not originally mine).heic".to_string();
    decode_heic(path);
    get_sun_pos(25.5, 32.3);
    Ok(())
}
