use std::path::Path;

use fontdue::Font;
use image::{Rgba, RgbaImage};

use crate::FontFillResult;

pub trait Canvas {
    fn size(&self) -> usize;
    fn decay(&mut self, decay: f32, min: f32);
    fn draw(&mut self, c: char, font: &Font, color: Rgba<f32>);
    fn as_png(&self) -> FontFillResult<RgbaImage>;
    fn as_buffer(&self) -> Vec<u8>;
    fn save(&self, path: &Path) -> FontFillResult<()> {
        let out: FontFillResult<()> = try {
            if path.exists() {
                remove_file(path)?
            }
            let mut buffer = RgbaImage::new(self.canvas.width(), self.canvas.height());
            for (x, y, pixel) in self.canvas.enumerate_pixels() {
                buffer.put_pixel(x, y, Rgba(pixel.0.map(|v| (v * 255.0) as u8)));
            }
            buffer.save(path)?
        };
        out.map_err(|e| e.with_path(path))
    }
    fn transparent_area(&self) -> f32;
}
