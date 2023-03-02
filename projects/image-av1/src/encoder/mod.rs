use std::{
    fs::{remove_file, File},
    io::ErrorKind,
    path::Path,
};

use image::ImageError;
use rav1e::{config::RateControlConfig, Config, EncoderConfig};

use crate::utils::io_error;

mod encoding;

pub struct Av1Encoder {
    output: File,
    config: Config,
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
        Ok(Self { output: File::create(path)?, config: Default::default() })
    }

    pub fn with_config(mut self, config: EncoderConfig) -> Self {
        self.config = self.config.with_encoder_config(config);
        self
    }
    pub fn with_rate_control(mut self, config: RateControlConfig) -> Self {
        self.config = self.config.with_rate_control(config);
        self
    }
    pub fn with_parallel_gops(mut self, config: usize) -> Self {
        self.config = self.config.with_parallel_gops(config);
        self
    }
}
