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
        // image::save_buffer(
        //     format!("image{}.png", id),
        //     img.planes().interleaved.unwrap().data,
        //     img.width(Channel::Interleaved).unwrap(),
        //     img.height(Channel::Interleaved).unwrap(),
        //     image::ColorType::Rgb8,
        // )
        // .unwrap();

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

        // let rgb_data: Rgb<&[u8]> = image::Rgb([data_r, data_g, data_b]);
        // let image = DynamicImage::ImageRgb8(rgb_data);

        let height = img.height(Channel::R)?;
        let width = img.width(Channel::R)?;

        // println!("length: {}", plane_r.data.len());
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

        // let len = 9266400;
        // let data = interleaved_plane.data;
        // let mut vec = data.to_vec();
        // vec.resize(len, 0);
        // let file = File::create("icon.png").unwrap();
        // let ref mut buff = BufWriter::new(file);
        // let encoder = PngEncoder::new(buff);

        // encoder
        //     .write_image(
        //         vec.as_bytes(),
        //         img.planes().interleaved.unwrap().width,
        //         img.planes().interleaved.unwrap().height,
        //         image::ColorType::Rgb8,
        //     )
        //     .unwrap();

        // println!("Image{} Finished", id);
        // panic!();

        // println!("{:?}", decode.bits_per_pixel(Channel::Interleaved).unwrap());
        // // Get pixels
        // let planes = decode.planes();
        // let interleaved_plane = planes.interleaved.unwrap();
        // // let data = interleaved_plane.data;
        // let data = img.planes().interleaved.unwrap();

        // println!("format {:?}", guess_format(data.data));
        // // img.planes().cb;
        // // img.planes().cr;

        // let file = File::create("icon.png").unwrap();
        // let ref mut buff = BufWriter::new(file);
        // let encoder = PngEncoder::new(buff);

        // encoder.write_image(
        //     bytes_y,
        //     img.planes().interleaved.unwrap().width,
        //     img.planes().interleaved.unwrap().height,
        //     image::ColorType::Rgb8,
        // );

        // for y in 0..img.planes().interleaved.unwrap().height {
        //     let mut bytes =
        //         Vec::with_capacity((img.planes().interleaved.unwrap().height as usize) * 3);
        //     for x in 0..(img.planes().interleaved.unwrap().width as usize) {
        //         let offset_y = (y * stride_y) as usize;
        //         bytes.push(bytes_y[offset_y + x]);
        //         let offset_u = ((y / 2) * stride_u) as usize;
        //         bytes.push(bytes_u[offset_u + x / 2]);
        //         let offset_v = ((y / 2) * stride_v) as usize;
        //         bytes.push(bytes_v[offset_v + x / 2]);
        //     }
        // }

        // save as png
        // std::fs::write( format!("output{}.png",id.to_string()), data).unwrap();
        // let mut buffer = File::create(format!("output{}.png", id.to_string())).unwrap();
        // // let height = interleaved_plane.height;
        // // let width = interleaved_plane.width;
        // let encoder = PngEncoder::new(buffer);
        // encoder.encode(bytes, width, height, color)
        // println!("{:?}", interleaved_plane.stride);
        // // encoder
        // //     .write_image(data, width, height, image::ColorType::)
        // //     .unwrap();

        // // Construct a new RGB ImageBuffer with the specified width and height.

        // let img: RgbImage = ImageBuffer::new(width, height);

        // Get metadata
        // let num_metadata = handle.number_of_metadata_blocks("");
        // let mut metadata_ids: Vec<ItemId> = vec![0; num_metadata as usize];
        // handle.metadata_block_ids("", &mut metadata_ids);
        // for meta_id in metadata_ids {
        //     let metadata = handle.metadata(meta_id)?;
        //     let value = xml::decode_xml(metadata, "apple_desktop").unwrap();
        //     let plist = list::decode_plist(value);
        // }
    }
    return Ok(1);
}
