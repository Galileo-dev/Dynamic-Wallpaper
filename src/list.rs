use base64::{decode};
use plist::Value;
use std::io::{Cursor, Result};

pub fn decode_plist(value: String) -> Result<()> {
  println!("value: {:?}", value);
  let base64 = decode(value).unwrap();
  // let decoded_plist = plist::Value::from(base64);
  // let values = decoded_plist.as_dictionary().unwrap();
  let base64_u8: &[u8] = &base64;
  println!("base64: {:?}", base64);
  let input = Cursor::new(base64_u8);

  println!("Cursor: {:?}", input);

  let book = Value::from_reader(input).expect("failed to read book.plist");
  println!("book : {:?}", book);
  Ok(())
}
