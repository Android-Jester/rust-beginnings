use std::vec;
use tokio;

use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageError,
    ImageFormat,
};
use image_combiner::combiner::FloatingImage;

use crate::image_combiner::combiner::Args;
mod calculator;
mod http_server;
mod image_combiner;

#[tokio::main]
fn main() -> Result<(), ImageDataErrors<'static>> {
    let args = Args::new();
    let (image_1, image_format_1) = find_image_from_path(args.image_1)?;
    let (image_2, image_format_2) = find_image_from_path(args.image_2)?;
    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }
    let (image_1, image_2) = standardize_size(image_1, image_2);
    let mut output = FloatingImage::new(image_1.width(), image_1.width(), args.output);
    output.set_data(combine_images(image_1, image_2))?;
    if let Err(e) = image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_format_1,
    ) {
        Err(ImageDataErrors::UnableToSaveImage(e))
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub enum ImageDataErrors<'a> {
    DifferentImageFormats,
    BufferTooSmall,
    UnableToReadImageFromPath(std::io::Error),
    UnableToFormatImage(&'a String),
    UnableToParseImage(ImageError),
    UnableToSaveImage(ImageError),
}

fn find_image_from_path<'a>(
    path: String,
) -> Result<(DynamicImage, ImageFormat), ImageDataErrors<'a>> {
    match Reader::open(path.clone()) {
        Ok(image_reader) => {
            if let Some(image_format) = image_reader.format() {
                match image_reader.decode() {
                    Ok(image) => Ok((image, image_format)),
                    Err(e) => Err(ImageDataErrors::UnableToParseImage(e)),
                }
            } else {
                Err(ImageDataErrors::BufferTooSmall)
            }
        }
        Err(e) => Err(ImageDataErrors::UnableToReadImageFromPath(e)),
    }
}

fn get_smallest_dimension(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim1.0 + dim1.1;
    let pix_2 = dim2.0 + dim2.1;
    if pix_1 < pix_2 {
        dim1
    } else {
        dim2
    }
}

fn standardize_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimension(image_1.dimensions(), image_2.dimensions());
    println!("Width: {}, height: {}", width, height);
    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();
    alternative_pixels(vec_1, vec_2)
}

fn alternative_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];
    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=(i + 3), set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=(i + 3), set_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val = match vec.get(i) {
            Some(data) => *data,
            None => panic!("Index is Out of Bounds"),
        };
        rgba.push(val)
    }
    rgba
}

