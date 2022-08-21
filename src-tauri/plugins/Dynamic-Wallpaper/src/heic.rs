use crate::{list, xml};
use base64::encode;
use image::{
    codecs::png::PngEncoder, guess_format, DynamicImage, EncodableLayout, GenericImage,
    GenericImageView, ImageBuffer, ImageEncoder, Rgb, RgbImage,
};
use libheif_rs::{Channel, Chroma, ColorSpace, HeifContext, ItemId, Result, RgbChroma};
use std::{
    fs::File,
    io::{BufWriter, Cursor, Read},
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
        let color_space = ColorSpace::Rgb(RgbChroma::C444);
        let img = handle.decode(color_space, false).unwrap();

        let planes = img.planes();
        let plane_r = planes.r.unwrap();
        let plane_g = planes.g.unwrap();
        let plane_b = planes.b.unwrap();
        let stride_r = plane_r.stride as u32;
        let stride_g = plane_g.stride as u32;
        let stride_b = plane_b.stride as u32;

        let data_r = plane_r.data;
        let data_g = plane_g.data;
        let data_b = plane_b.data;

        let height = img.height(Channel::R)?;
        let width = img.width(Channel::R)?;

        let mut new_img = Vec::<u8>::with_capacity(height as usize);
        for y in 0..height {
            let mut bytes = Vec::<u8>::with_capacity((width as usize) * 3);
            for x in 0..width as usize {
                let offset = (y * stride_r) as usize;
                bytes.push(data_r[offset + x]);
                bytes.push(data_g[offset + x]);
                bytes.push(data_b[offset + x]);
            }
            new_img.append(&mut bytes);
        }

        image::save_buffer(
            format!("image{}.png", id),
            new_img.as_bytes(),
            width,
            height,
            image::ColorType::Rgb8,
        )
        .unwrap();

        println!("new image length: {}", new_img.len());

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
