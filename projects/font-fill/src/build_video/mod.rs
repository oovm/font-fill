use std::{
    fs::{create_dir_all, read, remove_file, File},
    io::Write,
    path::{Path, PathBuf},
};

use fontdue::{Font, FontSettings};
use image::Rgba;
use rav1e::prelude::*;
use serde::Serialize as _;
use serde_derive::{Deserialize, Serialize};
use serde_json::Serializer;

use crate::{DecayCanvas, FontFillError, FontFillResult};

#[derive()]
pub struct FontFillVideo {
    canvas: DecayCanvas,
    directory: PathBuf,
    video: File,
    encoder: Context<u8>,
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
        let (video, directory) = create_video(video.as_ref())?;
        Ok(Self {
            // encode: Encoder::new((size, size), 4, Compression::Brotli(4), 30),
            canvas: DecayCanvas::new(size),
            directory,
            video,
            encoder: Config::default().new_context()?,
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

        let frame = self.encoder.new_frame();
        self.encoder.send_frame(frame)?;
        self.encoder.flush();
        loop {
            match self.encoder.receive_packet() {
                Ok(packet) => self.video.write_all(&packet.data)?,
                Err(EncoderStatus::Encoded) => continue,
                Err(EncoderStatus::LimitReached) => break,
                Err(err) => Err(err)?,
            }
        }
        Ok(fill_rate)
    }

    pub fn report_json(&self) -> FontFillResult<()> {
        let report = FontFillVideoReport { fill_rate: self.fill_rate.clone(), decay_rate: self.decay_rate, decay_min: self.decay_min };
        let file = self.directory.join("report.json");
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

fn create_video(path: &Path) -> FontFillResult<(File, PathBuf)> {
    let out: FontFillResult<(File, PathBuf)> = try {
        if path.exists() {
            log::info!("Removing existing file {}", path.display());
            remove_file(&path)?;
        }
        let parent = match path.parent() {
            Some(s) => {
                create_dir_all(s)?;
                s.canonicalize()?
            }
            None => Err(FontFillError::file_error("Could not get parent directory"))?,
        };

        (File::create(path)?, parent)
    };
    out.map_err(|e| e.with_path(path))
}
