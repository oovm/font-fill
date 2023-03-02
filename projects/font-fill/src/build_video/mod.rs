use std::{
    fs::{read, File},
    path::Path,
};

use fontdue::{Font, FontSettings};
use image::Rgba;
use serde::Serialize as _;
use serde_derive::{Deserialize, Serialize};
use serde_json::Serializer;

use image_av1::Av1Encoder;

use crate::{DecayCanvas, FontFillError, FontFillResult};

mod builder;

#[derive()]
pub struct FontFillVideo {
    canvas: DecayCanvas,
    encoder: Av1Encoder,
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
        let encode = Av1Encoder::new(video.as_ref(), true)?.with_size(size, size);
        Ok(Self {
            // encode: Encoder::new((size, size), 4, Compression::Brotli(4), 30),
            canvas: DecayCanvas::new(size),
            encoder: encode,
            font: load_font(font.as_ref())?,
            fill_rate: vec![],
            decay_rate: 0.7,
            decay_min: 0.2,
        })
    }
    pub fn encode_frame(&mut self, c: char, color: Rgba<f32>) -> FontFillResult<f32> {
        self.canvas.decay(self.decay_rate, self.decay_min);
        self.canvas.draw(c, &self.font, color);
        let fill_rate = 1.0 - self.canvas.transparent_area();
        self.fill_rate.push(fill_rate);
        self.encoder.encode_image_high_precision(self.canvas.get_image())?;
        Ok(fill_rate)
    }

    pub fn report_json(&self) -> FontFillResult<()> {
        let report = FontFillVideoReport { fill_rate: self.fill_rate.clone(), decay_rate: self.decay_rate, decay_min: self.decay_min };
        let file = self.encoder.get_directory().join("report.json");
        let file = File::create(file)?;
        let mut ser = Serializer::pretty(file);
        report.serialize(&mut ser)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontFillVideoReport {
    fill_rate: Vec<f32>,
    decay_rate: f32,
    decay_min: f32,
}

impl FontFillVideo {}

fn load_font(path: &Path) -> FontFillResult<Font> {
    try_load_font(path).map_err(|e| e.with_path(path))
}

fn try_load_font(path: &Path) -> FontFillResult<Font> {
    match Font::from_bytes(read(path)?, FontSettings::default()) {
        Ok(s) => Ok(s),
        Err(e) => Err(FontFillError::DecodeError { message: e.to_string() }),
    }
}
