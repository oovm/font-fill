use fontdue::{Font, FontSettings};
use image::{ImageBuffer, Rgba, RgbaImage};

pub use crate::build_video::FontFillVideo;
pub use crate::errors::FontFillError;

mod build_video;
mod errors;
mod build_image;

pub const TRANSPARENCY_THRESHOLD: f32 = 0.001;

pub struct FontFillCanvas {
    canvas: ImageBuffer<Rgba<f32>, Vec<f32>>,
}

impl FontFillCanvas {
    pub fn new(size: usize) -> Self {
        Self {
            canvas: ImageBuffer::new(size as u32, size as u32),
        }
    }
    pub fn size(&self) -> usize {
        self.canvas.width() as usize
    }
    pub fn decay(&mut self, decay: f32, min: f32) {
        for pixel in self.canvas.pixels_mut() {
            let alpha = pixel[3] * decay;
            if alpha <= min {
                continue;
            }
            pixel[3] = alpha;
        }
    }

    pub fn draw(&mut self, c: char, font: &Font, mut color: Rgba<f32>) {
        if font.lookup_glyph_index(c) == 0 {
            return;
        }
        let (metrics, bitmap) = font.rasterize(c, self.size() as f32);
        for x in 0..metrics.width {
            for y in 0..metrics.height {
                let index = y * metrics.width + x;
                let alpha = bitmap[index] as f32 / 255.0;
                if alpha <= TRANSPARENCY_THRESHOLD {
                    continue;
                }
                color[3] = alpha;
                self.canvas.put_pixel(x as u32, y as u32, color);
            }
        }
    }
    pub fn as_buffer(&self) -> Vec<u8> {
        self.canvas.iter().map(|f| (f * 255.0) as u8).collect()
    }

    pub fn save(&self, path: &str) {
        let mut buffer = RgbaImage::new(self.canvas.width(), self.canvas.height());
        for (x, y, pixel) in self.canvas.enumerate_pixels() {
            let alpha = (pixel[3] * 255.0) as u8;
            buffer.put_pixel(x, y, Rgba([255, 255, 255, alpha]));
        }
        buffer.save(path).unwrap();
    }
    pub fn transparent_area(&self) -> f32 {
        let mut count = 0;
        for pixel in self.canvas.pixels() {
            if pixel[3] <= TRANSPARENCY_THRESHOLD {
                count += 1;
            }
        }
        count as f32 / self.canvas.pixels().len() as f32
    }
}

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