use std::ops::Div;

use fontdue::{Font, FontSettings};
use image::RgbaImage;

pub struct FontCanvas {
    pub canvas: RgbaImage,
}

impl FontCanvas {
    pub fn new(size: usize) -> Self {
        Self {
            canvas: RgbaImage::new(size as u32, size as u32),
        }
    }
    pub fn size(&self) -> usize {
        self.canvas.width() as usize
    }
    pub fn draw(&mut self, c: char, font: &Font) {
        let size = self.size();
        let (metrics, bitmap) = font.rasterize(c, self.size() as f32);
        for x in 0..metrics.width {
            for y in 0..metrics.height {
                let px = x;
                let py = y;
                let index = y * metrics.width + x;
                let alpha = bitmap[index];
                self.canvas.put_pixel(px as u32, py as u32, image::Rgba([255, 255, 255, alpha]));
            }
        }
    }
    pub fn save(&self, path: &str) {
        self.canvas.save(path).unwrap();
    }
}

#[test]
fn test() {
// Read the font data.
    let font = include_bytes!(r"F:\project-a\client\ProjectA\Assets\Art\Fonts\SourceHanSansSC-Regular.otf") as &[u8];
// Parse it into the font type.
    let font = Font::from_bytes(font, FontSettings::default()).unwrap();

    let mut canvas = FontCanvas::new(144);
    canvas.draw('g', &font);
    canvas.save("test.png");
}

fn alignment_center(size: u32, min: i32, index: usize) -> u32 {
    let mut x = min + index as i32;
    x += size.div(2) as i32;
    if x <= 0 {
        0
    } else if x >= size as i32 {
        size - 1
    } else {
        x as u32
    }
}