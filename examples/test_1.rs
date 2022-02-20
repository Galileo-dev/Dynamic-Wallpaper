extern crate plist;
#[macro_use]
extern crate serde_derive;

use plist::Value;
use serde::{Deserialize, Serialize};

use base64::{decode, encode};
use std::io::{Cursor, Read};
use xml::reader::{EventReader, XmlEvent};
fn main() -> Result<()> {
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct decoded_metadata {}

    let path = "C:/Users/fionn/Documents/anime room (not originally mine).heic".to_string();
    println!("decoding");
    let ctx = HeifContext::read_from_file(&path)?;
    let num_images = ctx.number_of_top_level_images();
    let mut image_ids: Vec<ItemId> = vec![0; num_images];
    let count = ctx.top_level_image_ids(&mut image_ids);
    println!("{:?}", image_ids);
    for id in image_ids {
        let handle = ctx.image_handle(id)?;

        // Get metadata
        let num_metadata = handle.number_of_metadata_blocks("");
        let mut metadata_ids: Vec<ItemId> = vec![0; num_metadata as usize];
        handle.metadata_block_ids("", &mut metadata_ids);
        for meta_id in metadata_ids {
            let metadata = handle.metadata(meta_id)?;
            let xml = EventReader::from_str(std::str::from_utf8(&metadata).unwrap());
            let mut depth = 0;
            let mut value = String::new();
            for e in xml {
                match e {
                    Ok(XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    }) => {
                        // println!("Name: {:?}", name);
                        // println!("Attributes: {:?}", attributes);
                        // println!("Namespaces: {:?}", namespace);

                        for attribute in attributes {
                            if attribute.name.prefix == Some(filter) {
                                println!("{}", attribute.value);
                                value = attribute.value;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
            println!("value: {:?}", value);
            let base64 = decode(value).unwrap();
            // let decoded_plist = plist::Value::from(base64);
            // let values = decoded_plist.as_dictionary().unwrap();
            let base64_u8: &[u8] = &base64;
            println!("base64: {:?}", base64);
            let mut input = Cursor::new(base64_u8);

            println!("Cursor: {:?}", input);

            let book = Value::from_reader(input).expect("failed to read book.plist");
            println!("book : {:?}", book);
            let title = book.as_dictionary();
            println!("title : {:?}", title);

            // let p_list = plist::from_reader(&mut input).unwrap();
            // println!("decoded_plist: {:?}", p_list)

            // println!("handle: {:?}", metadata);

            let unixtime = 1362441600000;
            let lat = 48.0;
            let lon = 9.0;
            let pos = sun::pos(unixtime, lat, lon);
            let az = pos.azimuth.to_degrees();
            let alt = pos.altitude.to_degrees();
            println!("The position of the sun is {}/{}", az, alt);
        }
    }

    Ok(())
}
