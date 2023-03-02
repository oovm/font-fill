use std::{
    fs::{create_dir_all, remove_file, File},
    io::ErrorKind,
    path::{Path, PathBuf},
};

use image::ImageError;
use rav1e::{color::ChromaSampling, config::RateControlConfig, data::Rational, EncoderConfig};

use crate::utils::io_error;

mod encoding;

pub struct Av1Encoder {
    path: PathBuf,
    output: File,
    config: EncoderConfig,
    rate_control: RateControlConfig,
}

impl Av1Encoder {
    /// Export images to file
    ///
    /// # Arguments
    ///
    /// * `output`:
    /// * `overwrite`:
    ///
    /// returns: Result<Av1Encoder, ImageError>
    ///
    /// # Examples
    ///
    /// ```
    /// # use image_av1::Av1Encoder;
    /// # #[allow(unused_must_use)]
    /// Av1Encoder::new("output.ivf", true);
    /// ```
    pub fn new<P: AsRef<Path>>(output: P, overwrite: bool) -> Result<Self, ImageError> {
        let path = output.as_ref();
        if path.exists() {
            if overwrite {
                remove_file(path)?;
            }
            else {
                io_error(ErrorKind::AlreadyExists, format!("File already exists: {}", path.display()))?;
            }
        }
        if let Some(s) = path.parent() {
            create_dir_all(s)?;
        }
        let output = File::create(path)?;
        let mut encoder = EncoderConfig::default();
        encoder.chroma_sampling = ChromaSampling::Cs444;
        Ok(Self { output, path: path.canonicalize()?, config: encoder, rate_control: Default::default() })
    }
    pub fn get_file_path(&self) -> &Path {
        self.path.as_ref()
    }
    pub fn get_directory(&self) -> &Path {
        match self.path.parent() {
            None => self.get_file_path(),
            Some(s) => s,
        }
    }
    pub fn mut_config(&mut self) -> &mut EncoderConfig {
        &mut self.config
    }
    pub fn with_config(mut self, config: EncoderConfig) -> Self {
        self.config = config;
        self
    }
    pub fn with_size(mut self, width: usize, height: usize) -> Self {
        self.config.width = width;
        self.config.height = height;
        self
    }
    pub fn with_fps(mut self, fps: usize) -> Self {
        self.config.time_base = Rational { num: 1, den: fps as u64 };
        self
    }
    pub fn with_rate_control(mut self, rate_control: RateControlConfig) -> Self {
        self.rate_control = rate_control;
        self
    }
    // pub fn with_parallel_gops(mut self, config: usize) -> Self {
    //     self.encoder = self.encoder.with_parallel_gops(config);
    //     self
    // }
}
