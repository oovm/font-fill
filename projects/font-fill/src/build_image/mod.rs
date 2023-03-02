use std::{fs::remove_file, path::Path};

use fontdue::Font;
use image::{ImageBuffer, Rgba, RgbaImage};

use crate::FontFillResult;

pub const TRANSPARENCY_THRESHOLD: f32 = 0.001;

pub struct DecayCanvas {
    buffer: ImageBuffer<Rgba<f32>, Vec<f32>>,
}

impl DecayCanvas {
    pub fn new(size: usize) -> Self {
        Self { buffer: ImageBuffer::new(size as u32, size as u32) }
    }
    pub fn size(&self) -> usize {
        self.buffer.width() as usize
    }
    pub fn decay(&mut self, decay: f32, min: f32) {
        for pixel in self.buffer.pixels_mut() {
            let alpha = pixel[3] * decay;
            if alpha <= min {
                continue;
            }
            pixel[3] = alpha;
        }
    }

    pub fn draw(&mut self, c: char, font: &Font, mut color: Rgba<f32>) {
        let (metrics, bitmap) = match font.lookup_glyph_index(c) {
            0 => {
                log::error!("No glyph for {}", c);
                return;
            }
            code => font.rasterize_indexed(code, self.size() as f32),
        };
        for x in 0..metrics.width {
            for y in 0..metrics.height {
                let px = self.size() / 2 - metrics.width as usize / 2 + x as usize;
                let py = self.size() / 2 - metrics.height as usize / 2 + y as usize;
                if px >= self.size() || py >= self.size() {
                    log::error!("({}, {}) out of bounds: ({}, {})", px, py, self.size(), self.size());
                    continue;
                }
                let index = y * metrics.width + x;
                let alpha = bitmap[index] as f32 / 255.0;
                if alpha <= TRANSPARENCY_THRESHOLD {
                    continue;
                }
                color[3] = alpha;
                self.buffer.put_pixel(px as u32, py as u32, color);
            }
        }
    }
    pub fn get_image(&self) -> &ImageBuffer<Rgba<f32>, Vec<f32>> {
        &self.buffer
    }

    pub fn as_buffer(&self) -> Vec<u8> {
        self.buffer.iter().map(|f| (f * 255.0) as u8).collect()
    }

    pub fn save(&self, path: &Path) -> FontFillResult<()> {
        let out: FontFillResult<()> = try {
            if path.exists() {
                remove_file(path)?
            }
            let mut buffer = RgbaImage::new(self.buffer.width(), self.buffer.height());
            for (x, y, pixel) in self.buffer.enumerate_pixels() {
                buffer.put_pixel(x, y, Rgba(pixel.0.map(|v| (v * 255.0) as u8)));
            }
            buffer.save(path)?
        };
        out.map_err(|e| e.with_path(path))
    }
    pub fn transparent_area(&self) -> f32 {
        let mut count = 0;
        for pixel in self.buffer.pixels() {
            if pixel[3] <= TRANSPARENCY_THRESHOLD {
                count += 1;
            }
        }
        count as f32 / self.buffer.pixels().len() as f32
    }
}
