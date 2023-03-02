use fill_fonts::{DecayCanvas, FontFillResult, FontFillVideo};
use fontdue::{Font, FontSettings};
use image::Rgba;
use std::path::PathBuf;

#[test]
pub fn ready() {
    print!("it's ready!")
}

#[test]
fn test() {
    // Read the font data.
    let font = include_bytes!(r"F:\project-a\client\ProjectA\Assets\Art\Fonts\SourceHanSansSC-Regular.otf") as &[u8];
    // Parse it into the font type.
    let font = Font::from_bytes(font, FontSettings::default()).unwrap();

    let mut canvas = DecayCanvas::new(144);
    canvas.draw('生', &font, Rgba([1.0, 0.0, 0.0, 1.0]));
    canvas.decay(0.7, 0.1);
    canvas.draw('僻', &font, Rgba([1.0, 0.0, 0.0, 1.0]));
    canvas.decay(0.7, 0.1);
    canvas.draw('字', &font, Rgba([1.0, 0.0, 0.0, 1.0]));
    canvas.save(&PathBuf::from("test.png")).unwrap();
}
