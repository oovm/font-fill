use image_av1::Av1Encoder;

#[test]
fn test() {
    Av1Encoder::new("output.ivf", true).unwrap();
}
