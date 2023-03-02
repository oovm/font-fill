use image::DynamicImage;
use image_av1::Av1Encoder;

#[test]
fn test() {
    let image = DynamicImage::new_rgb8(144, 144);

    let mut encoder = Av1Encoder::new("output.ivf", true).unwrap().with_size(144, 144);
    encoder.write_image(&image.to_rgba8()).unwrap();
}
