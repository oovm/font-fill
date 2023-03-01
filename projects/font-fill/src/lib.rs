use fontdue::{Font, FontSettings};
use image::{Rgba};

pub use crate::build_video::FontFillVideo;
pub use crate::errors::{FontFillError, FontFillResult};
pub use crate::build_image::FontFillCanvas;

mod build_video;
mod errors;
mod build_image;



#[test]
fn test() {
// Read the font data.
    let font = include_bytes!(r"F:\project-a\client\ProjectA\Assets\Art\Fonts\SourceHanSansSC-Regular.otf") as &[u8];
// Parse it into the font type.
    let font = Font::from_bytes(font, FontSettings::default()).unwrap();

    let mut canvas = FontFillCanvas::new(144);
    canvas.draw('生', &font, Rgba([1.0, 0.0, 0.0, 1.0]));
    canvas.decay(0.7, 0.1);
    canvas.draw('僻', &font, Rgba([1.0, 0.0, 0.0, 1.0]));
    canvas.decay(0.7, 0.1);
    canvas.draw('字', &font, Rgba([1.0, 0.0, 0.0, 1.0]));
    canvas.save("test.png");
}


#[test]
fn main() {
    let mut file = FontFillVideo::create(r"F:\project-a\client\ProjectA\Assets\Art\Fonts\SourceHanSansSC-Regular.otf", "test.mp4", 144);
    file.encode_frame('生', Rgba([1.0, 0.0, 0.0, 1.0]));
    file.encode_frame('僻', Rgba([1.0, 0.0, 0.0, 1.0]));
    file.encode_frame('字', Rgba([1.0, 0.0, 0.0, 1.0]));
}