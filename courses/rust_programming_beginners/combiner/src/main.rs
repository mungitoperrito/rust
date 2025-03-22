mod args;
use args::Args;
use std::{io::BufReader, fs::File};
use image::{imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat};


#[derive{Debug}]enum ImageDataErrors {
  DifferentImageFormats,
}


enum ImageDataErrors{

}

struct TempImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl TempImage {
    fn new(width: u32, height: u32, name: String) -> Self {
      let pixel_value = 4;
      let buffer_capacity = height * width * pixel_value;
      let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
      TempImage {
        width,
        height,
        data: buffer,
        name
      }
    }
}

fn main() -> Result<(), ImageDataErrors>{
  let args = Args::new();
  // // Uncomment to debug
  // println!("{:?}", args)

  let(image_01, image_format_01) = find_image_from_path(args.image_01);
  let(image_02, image_format_02) = find_image_from_path(args.image_02);

  if image_format_01 != image_format_02 {
    return Err(ImageDataErrors::DifferentImageFormats);
  }

  let(image_01, image_02) = standardize_size(image_01, image_02);

  Ok(())                      // Empty return on success
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)    // return
}

fn get_smallest_dimensions(dimension_01: (u32, u32), dimension_02: (u32, u32))
                           -> (u32, u32) {
  let pix_01 = dimension_01.0 * dimension_01.1;
  let pix_02 = dimension_02.0 * dimension_02.1;

  return if pix_01 < pix_02 {dimension_01} else {dimension_02};
}

fn standardize_size(image_01: DynamicImage, image_02: DynamicImage)
                    -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_01.dimensions(), image_02.dimensions());
  println!("w: {}, h: {}\n", width, height);

  if image_02.dimensions() == (width, height) {
    (image_01.resize_exact(width, height, Triangle), image_02)
  } else {
    (image_02.resize_exact(width, height, Triangle), image_01)
  }


}