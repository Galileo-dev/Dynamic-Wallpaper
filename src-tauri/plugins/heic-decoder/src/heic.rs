use crate::{list, xml};
use base64::encode;
use color_eyre::Result;
use image::{
  codecs::png::PngEncoder, guess_format, DynamicImage, EncodableLayout, GenericImage,
  GenericImageView, ImageBuffer, ImageEncoder, Rgb, RgbImage,
};
use libheif_rs::{Channel, Chroma, ColorSpace, HeifContext, ItemId, RgbChroma};
use rayon::{prelude::*, result};
use std::{path::PathBuf, thread::current, u8};

#[allow(warnings)]
pub fn decode_heic(file_path: PathBuf, export_path: PathBuf) -> Result<()> {
  println!("decoding");

  let ctx = HeifContext::read_from_file(&file_path.to_str().unwrap())?;
  let num_images = ctx.number_of_top_level_images();
  let mut image_ids: Vec<ItemId> = vec![0; num_images];
  let count = ctx.top_level_image_ids(&mut image_ids);
  println!("{:?}", image_ids);
  for id in image_ids {
    let handle = ctx.image_handle(id)?;
    let color_space = ColorSpace::Rgb(RgbChroma::Rgb);
    let img = handle.decode(color_space, false).unwrap();

    let planes = img.planes();
    // let (bytes_r, stride_r) = planes.r.map(|x| (x.data, x.stride)).unwrap();
    // let (bytes_g, stride_g) = planes.g.map(|x| (x.data, x.stride)).unwrap();
    // let (bytes_b, stride_b) = planes.b.map(|x| (x.data, x.stride)).unwrap();
    let (bytes_inter, stride_inter) = planes.interleaved.map(|x| (x.data, x.stride)).unwrap();

    println!("bytes inter: {:?}", bytes_inter.len());
    let height = img.height(Channel::Interleaved)?;
    let width = img.width(Channel::Interleaved)?;
    let mut export_file_path = export_path.clone();
    export_file_path.push(format!(r"{}.png", id));
    image::save_buffer(
      &export_file_path,
      bytes_inter.as_bytes(),
      width,
      height,
      image::ColorType::Rgb8,
    )?;

    // let mut result = Vec::<Vec<_>>::with_capacity(height as usize);
    // let mut result: Vec<u8> = Vec::with_capacity(width as usize * height as usize * 3);
    // result.par_iter_mut().enumerate().for_each(|(i, pixel)| {
    //   let offset = (y * stride_r) as usize;
    //   pixel.push(bytes_r[offset + x]);
    //   pixel.push(bytes_g[offset + x]);
    //   pixel.push(bytes_b[offset + x]);
    // });

    // let mut result = vec![vec![vec![]; width as usize]; height as usize];
    // result
    //   .par_iter_mut()
    //   .enumerate()
    //   .map(|(y, row)| {
    //     row
    //       .par_iter_mut()
    //       .enumerate()
    //       .map(|(x, pixel)| {
    //         let offset = (y * stride_r) as usize;
    //         pixel.push(bytes_r[offset + x]);
    //         pixel.push(bytes_g[offset + x]);
    //         pixel.push(bytes_b[offset + x]);
    //       })
    //       .collect::<Vec<_>>();
    //   })
    //   .collect::<Vec<_>>();
    // println!("image {} done processing", id);
    // let bytes = result
    //   .into_par_iter()
    //   .flatten()
    //   .flatten()
    //   .collect::<Vec<u8>>();

    // println!("bytes: {:?}", bytes.len());
    // println!("width is {} and height is {}", width, height);
    // println!("export_path");

    // let mut export_file_path = export_path.clone();
    // export_file_path.push(format!(r"{}.png", id));
    // image::save_buffer(
    //   &export_file_path,
    //   bytes.as_bytes(),
    //   width,
    //   height,
    //   image::ColorType::Rgb8,
    // )
    // .unwrap();

    // let mut height_vec: Vec<u8> = (0..height).collect::<Vec<_>>();

    // height_vec.par_iter_mut().for_each(|p| {
    //   let mut width_vec = (0..width).collect::<Vec<_>>();
    //   width_vec.par_iter_mut().enumerate().for_each(|(j, k)| {})
    // })

    // let result = (0..height as usize).into_par_iter().for_each(|i| {
    //   let mut intermediate_result = vec![];
    //   (0..width as usize)
    //     .into_par_iter()
    //     .map(|j| {})
    //     .collect_into(&mut intermediate_result);
    //   {}
    // });

    // println!("result: {:?}", result);

    // let plane_g = planes.g.unwrap();
    // let plane_b = planes.b.unwrap();
    // let stride_r = plane_r.stride as u32;
    // let stride_g = plane_g.stride as u32;
    // let stride_b = plane_b.stride as u32;

    // let data_r = plane_r.data;
    // let data_g = plane_g.data;
    // let data_b = plane_b.data;

    // let height = img.height(Channel::R)?;
    // let width = img.width(Channel::R)?;

    // let mut new_img = Vec::<u8>::with_capacity(height as usize);
    // for y in 0..height {
    //   let mut bytes = Vec::<u8>::with_capacity((width as usize) * 3);
    //   for x in 0..width as usize {
    //     let offset = (y * stride_r) as usize;
    //     bytes.push(data_r[offset + x]);
    //     bytes.push(data_g[offset + x]);
    //     bytes.push(data_b[offset + x]);
    //   }
    //   new_img.append(&mut bytes);
    // }
    // let export_path_full = format!(r"{}\{}.png", export_path, id);
    // println!("export_path_full = {}", export_path_full);

    // println!("new image length: {}", new_img.len());

    // Get metadata
    let num_metadata = handle.number_of_metadata_blocks("");
    let mut metadata_ids: Vec<ItemId> = vec![0; num_metadata as usize];
    handle.metadata_block_ids("", &mut metadata_ids);
    for meta_id in metadata_ids {
      let metadata = handle.metadata(meta_id)?;
      let value = xml::decode_xml(metadata, "apple_desktop").unwrap();
      let plist = list::decode_plist(value).unwrap();
    }
  }
  return Ok(());
}
