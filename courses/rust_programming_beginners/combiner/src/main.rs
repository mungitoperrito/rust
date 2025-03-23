mod args;
use args::Args;
use image::{imageops::FilterType::Triangle, io::Reader, DynamicImage,
            GenericImageView, ImageError, ImageFormat};
use std::convert::TryInto;

#[derive(Debug)]
enum ImageDataErrors {
    BufferTooSmall,
    DifferentImageFormats,
    UnableToDecodeImage(ImageError),
    UnableToFormatImage(String),
    UnableToReadImageFromPath(std::io::Error),
    UnableToSaveImage(ImageError)
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
      let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity.try_into().unwrap());
      TempImage {
        width,
        height,
        data: buffer,
        name
      }
    }


    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;

        Ok(())                  // Return doesn't need ;
    }
}


fn find_image_from_path(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataErrors> {
    match Reader::open(&path){
        Ok(image_reader) =>{
            if let Some(image_format) = image_reader.format() {
            match image_reader.decode(){
                Ok(image) => Ok((image, image_format)),
                Err(e) => Err(ImageDataErrors::UnableToDecodeImage(e))
                }
            } else {
                return Err(ImageDataErrors::UnableToFormatImage(path));
            }
        },
        Err(e) => Err(ImageDataErrors::UnableToReadImageFromPath(e))
    }
}


fn get_smallest_dimensions(dimension_01: (u32, u32), dimension_02: (u32, u32))
                           -> (u32, u32) {
  let pix_01 = dimension_01.0 * dimension_01.1;
  let pix_02 = dimension_02.0 * dimension_02.1;

  return if pix_01 < pix_02 {dimension_01} else {dimension_02};
}


fn standardize_size(image_01: DynamicImage,
                    image_02: DynamicImage)
                    -> (DynamicImage, DynamicImage) {
  let (width, height) =
     get_smallest_dimensions(image_01.dimensions(), image_02.dimensions());
    //  // Uncomment to debug
    //  println!("w: {}, h: {}\n", width, height);

  if image_02.dimensions() == (width, height) {
    (image_01.resize_exact(width, height, Triangle), image_02)
  } else {
    (image_01, image_02.resize_exact(width, height, Triangle))
  }
}


fn combine_images(image_01: DynamicImage,
                  image_02: DynamicImage)
                  -> Vec<u8> {
    let vec_01 = image_01.to_rgb8().into_vec();
    let vec_02 = image_02.to_rgb8().into_vec();

    alternate_pixels(vec_01, vec_02)
}


fn alternate_pixels(vec_01: Vec<u8>, vec_02: Vec<u8>) -> Vec<u8> {
    let mut combined = vec![0u8; vec_01.len()];

    let mut i = 0;
    while i < vec_01.len() {
        if i % 8 == 0 {
            combined.splice(i..= i + 3, set_rgba(&vec_01, i, i + 3));
        } else {
            combined.splice(i..= i + 3, set_rgba(&vec_02, i, i + 3));
        }
        i += 4;         // Size of one pixel
    }

    combined  // Return

}


fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8>{
    let mut rgba = Vec::new();
    for i in start..=end {
        // Avoid null values
        let val = match vec.get(i) {
            Some(d) => *d,            // *d dereferences the value of d
            None => panic!("ERR: Index out of bounds")
        };
        rgba.push(val);
    }
    rgba
}


//############
//### MAIN ###
//############

fn main() -> Result<(), ImageDataErrors>{
    let args = Args::new();

    let(image_01, image_format_01) = find_image_from_path(args.image_01)?;
    let(image_02, image_format_02) = find_image_from_path(args.image_02)?;

    if image_format_01 != image_format_02 {
      return Err(ImageDataErrors::DifferentImageFormats);
    }

    let(image_01, image_02) = standardize_size(image_01, image_02);
    let mut output = TempImage::new(image_01.width(), image_01.height(), args.output);

    let combined = combine_images(image_01, image_02);

    output.set_data(combined)?;

    // Fails here. Not sure why. Tried with same fotos. Don't know where none
    //    is coming from.
    if let Err(e) = image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgb8,
        image_format_01,
    ) {
        Err(ImageDataErrors::UnableToSaveImage(e))
    } else {
        Ok(())                      // Empty return on success
    }
  }