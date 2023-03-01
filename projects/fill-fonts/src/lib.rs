use fontdue::{Font, FontSettings};
use image::RgbaImage;

#[test]
fn test() {
// Read the font data.
    let font = include_bytes!(r"F:\project-a\client\ProjectA\Assets\Art\Fonts\SourceHanSansSC-Regular.otf") as &[u8];
// Parse it into the font type.
    let font = Font::from_bytes(font, FontSettings::default()).unwrap();
// Rasterize and get the layout metrics for the letter 'g' at 17px.
    let (metrics, bitmap) = font.rasterize('g', 144.0);

    let mut canvas = RgbaImage::new(144, 144);
    for x in 0..metrics.width {
        for y in 0..metrics.height {
            let px = metrics.xmin + x;
            let py = metrics.ymin + y;
            let index = (y * metrics.width + x) as usize;
            let alpha = bitmap[index];
            canvas.put_pixel(px as u32, py as u32, image::Rgba([255, 255, 255, alpha]));
        }
    }
    canvas.save("test.png").unwrap();
}