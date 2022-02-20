use crate::{list, xml};
use libheif_rs::{Channel, ColorSpace, HeifContext, ItemId, Result, RgbChroma};
use std::io::{Cursor, Read};
pub fn decode_heic(path: String) -> Result<(i32)> {
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
            let value = xml::decode_xml(metadata, "apple_desktop").unwrap();
            let plist = list::decode_plist(value);
        }
    }
    return Ok(1);
}
