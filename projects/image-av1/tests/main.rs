use image::{io::Reader as ImageReader, DynamicImage, ImageError};
use image_av1::Av1Encoder;
use std::io::Cursor;

#[test]
fn test() -> Result<(), ImageError> {
    let img = ImageReader::open("105802666_p0_master1200.jpg")?.decode()?;
    // let img2 = ImageReader::new(Cursor::new(img)).with_guessed_format()?.decode()?;
    let mut encoder = Av1Encoder::new("output.ivf", true).unwrap().with_size(img.width() as usize, img.height() as usize);
    encoder.encode_image(&img).unwrap();
    Ok(())
}
