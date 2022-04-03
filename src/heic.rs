use crate::{list, xml};
use image::{
    codecs::png::PngEncoder, GenericImage, GenericImageView, ImageBuffer, ImageEncoder, RgbImage,
};
use libheif_rs::{Channel, ColorSpace, HeifContext, ItemId, Result, RgbChroma};
use std::{
    fs::File,
    io::{Cursor, Read},
};
pub fn decode_heic(path: String) -> Result<(i32)> {
    println!("decoding");
    let ctx = HeifContext::read_from_file(&path)?;
    let num_images = ctx.number_of_top_level_images();
    let mut image_ids: Vec<ItemId> = vec![0; num_images];
    let count = ctx.top_level_image_ids(&mut image_ids);
    println!("{:?}", image_ids);
    for id in image_ids {
        let handle = ctx.image_handle(id)?;
        let color_space = ColorSpace::Rgb(RgbChroma::Rgb);
        let decode = handle.decode(color_space, false).unwrap();
        println!("{:?}", decode.bits_per_pixel(Channel::Interleaved).unwrap());
        // Get pixels
        let planes = decode.planes();
        let interleaved_plane = planes.interleaved.unwrap();
        let data = interleaved_plane.data;

        // save as png
        // std::fs::write( format!("output{}.png",id.to_string()), data).unwrap();
        let mut buffer = File::create(format!("output{}.png", id.to_string())).unwrap();
        let height = interleaved_plane.height;
        let width = interleaved_plane.width;
        let encoder = PngEncoder::new(buffer);
        println!("{:?}", interleaved_plane.stride);
        encoder
            .write_image(data, width, height, image::ColorType::L8)
            .unwrap();

        // Construct a new RGB ImageBuffer with the specified width and height.

        let img: RgbImage = ImageBuffer::new(width, height);

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
