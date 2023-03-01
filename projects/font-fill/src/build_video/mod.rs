use std::{
    fs::{File, read, remove_file},
    path::Path,
};
use std::fs::create_dir_all;

use fontdue::{Font, FontSettings};
use image::Rgba;
use syeve::{Compression, Encoder};

use crate::{FontFillCanvas, FontFillError, FontFillResult};

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
    pub fn create<V, F>(video: V, font: F, size: usize) -> FontFillResult<Self>
    where
        V: AsRef<Path>,
        F: AsRef<Path>,
    {

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
    let bytes = read(path)?;
    match Font::from_bytes(bytes, FontSettings::default()) {
        Ok(s) => Ok(s),
        Err(e) => Err(FontFillError::DecodeError { message: e.to_string() }),
    }
}

fn load_video(path: &Path) -> FontFillResult<File> {
    if path.exists() {
        if let Err(e) = remove_file(&path) {
            Err(FontFillError::FileError {
                path: path.display().to_string(),
                message: e.to_string(),
            })?
        }
    }
    let parent = path.parent().unwrap();

    create_dir_all(path.parent().unwrap()).unwrap(
    File::create(path).unwrap()
}