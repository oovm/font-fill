use std::fs::{File, remove_file};
use std::path::Path;
use fontdue::{Font, FontSettings};
use image::Rgba;

use syeve::{Compression, Encoder};

use crate::{FontFillCanvas, FontFillError};

#[derive()]
pub struct FontFillVideo {
    encode: Encoder,
    canvas: FontFillCanvas,
    file: File,
    font: Font,
    fill_rate: Vec<f32>,
    decay_rate: f32,
    decay_min: f32,
}


impl FontFillVideo {
    pub fn create<V, F>(video: V, font: F, size: usize) -> Self where V: AsRef<Path>, F: AsRef<Path> {
        let path = video.as_ref().to_path_buf();
        let file = if !path.exists() {
            File::create(path).unwrap()
        } else {
            // delete old file
            remove_file(&path).unwrap();
            File::create(path).unwrap()
        };
        let font = Font::from_bytes(std::fs::read(font.as_ref()).unwrap(), FontSettings::default()).unwrap();
        Self {
            encode: Encoder::new((size, size), 4, Compression::Brotli(4), 30),
            canvas: FontFillCanvas::new(size),
            file,
            font,
            fill_rate: vec![],
            decay_rate: 0.7,
            decay_min: 0.1,
        }
    }
    pub fn encode_frame(&mut self, c: char, color: Rgba<f32>) {
        self.canvas.decay(self.decay_rate, self.decay_min);
        self.canvas.draw(c, &self.font, color);
        self.fill_rate.push(1.0 - self.canvas.transparent_area());
        self.encode.encode(&mut self.canvas.as_buffer(), &mut self.file).unwrap();
    }
}

impl FontFillVideo {

}
pub fn get_font(path: &Path) -> Result<Font, FontFillError> {
    &self.font
}