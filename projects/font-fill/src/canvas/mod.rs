use std::{fs::remove_file, path::Path};

use fontdue::Font;
use image::{Rgba, RgbaImage};

use crate::FontFillResult;

pub trait Canvas {
    fn size(&self) -> usize;
    fn decay(&mut self, decay: f32, min: f32);
    fn draw(&mut self, c: char, font: &Font, color: Rgba<f32>);
    fn as_png(&self) -> RgbaImage;
    fn as_buffer(&self) -> Vec<u8>;
    fn save(&self, path: &Path) -> FontFillResult<()> {
        let out: FontFillResult<()> = try {
            if path.exists() {
                log::info!("Removing existing file {}", path.display());
                remove_file(path)?
            }
            let mut buffer = RgbaImage::new(self.size() as u32, self.size() as u32);
            self.as_png().save(path)?
        };
        out.map_err(|e| e.with_path(path))
    }
    fn transparent_area(&self) -> f32;
}
