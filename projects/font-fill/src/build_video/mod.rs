use std::fs::{File, read, remove_file};
use std::path::Path;

use fontdue::{Font, FontSettings};
use image::Rgba;
use syeve::{Compression, Encoder};

use crate::{FontFillCanvas, FontFillError, FontFillResult};
use crate::FontFillError::DecodeError;

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
    pub fn create<V, F>(video: V, font: F, size: usize) -> FontFillResult<Self> where V: AsRef<Path>, F: AsRef<Path> {
        let path = video.as_ref().to_path_buf();
        let file = if !path.exists() {
            File::create(path).unwrap()
        } else {
            // delete old file
            remove_file(&path).unwrap();
            File::create(path).unwrap()
        };
        Ok(Self {
            encode: Encoder::new((size, size), 4, Compression::Brotli(4), 30),
            canvas: FontFillCanvas::new(size),
            file,
            font: load_font(font.as_ref())?,
            fill_rate: vec![],
            decay_rate: 0.7,
            decay_min: 0.1,
        })
    }
    pub fn encode_frame(&mut self, c: char, color: Rgba<f32>) {
        self.canvas.decay(self.decay_rate, self.decay_min);
        self.canvas.draw(c, &self.font, color);
        self.fill_rate.push(1.0 - self.canvas.transparent_area());
        self.encode.encode(&mut self.canvas.as_buffer(), &mut self.file).unwrap();
    }
}

impl FontFillVideo {}

fn load_font(path: &Path) -> FontFillResult<Font> {
    let bytes = match read(path) {
        Ok(e) => {
            e
        }
        Err(e) => {
            FontFillError::FileError {
                path: path.to_string(),
                message: e.to_string(),
            }
        }
    };
    match Font::from_bytes(bytes, FontSettings::default()) {
        Ok(s) => { Ok(s) }
        Err(e) => {
            DecodeError {
                message: e.to_string(),
            }
        }
    }
}