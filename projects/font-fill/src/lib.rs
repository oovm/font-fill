#![feature(once_cell)]
#![feature(try_blocks)]

pub use crate::{
    build_image::DecayCanvas,
    build_video::FontFillVideo,
    errors::{FontFillError, FontFillResult},
};

mod build_image;
mod build_video;
mod canvas;
mod errors;
