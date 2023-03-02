use std::io::Write;

use image::{
    error::{ParameterError, ParameterErrorKind},
    imageops::FilterType,
    DynamicImage, ImageError, Rgba, Rgba32FImage, RgbaImage,
};
use rav1e::prelude::*;

use crate::{utils::Result, Av1Encoder};

impl Av1Encoder {
    pub fn resize_image(&mut self, image: DynamicImage, filter: FilterType) -> RgbaImage {
        image.resize_exact(self.config.width as u32, self.config.height as u32, filter).to_rgba8()
    }

    pub fn encode_image(&mut self, image: &RgbaImage) -> Result<usize> {
        self.check_size(image.width(), image.height())?;
        let mut size = 0;
        let mut ctx = self.encode_context()?;
        let mut frame = ctx.new_frame();
        self.build_rgba8_frame(&image, &mut frame)?;
        ctx.send_frame(frame).unwrap();
        ctx.flush();
        loop {
            match ctx.receive_packet() {
                Ok(packet) => {
                    size = self.output.write(&packet.data)?;
                    continue;
                }
                Err(EncoderStatus::Encoded) => continue,
                Err(EncoderStatus::LimitReached) => break,
                Err(err) => Err(err).unwrap(),
            }
        }
        Ok(size)
    }
    pub fn encode_image_high_precision(&mut self, image: &Rgba32FImage) -> Result<usize> {
        self.check_size(image.width(), image.height())?;
        let mut size = 0;
        let mut ctx = self.encode_context()?;
        let mut frame = ctx.new_frame();
        self.build_rgba32_frame(&image, &mut frame)?;
        ctx.send_frame(frame).unwrap();
        ctx.flush();
        loop {
            match ctx.receive_packet() {
                Ok(packet) => {
                    size = self.output.write(&packet.data)?;
                    continue;
                }
                Err(EncoderStatus::Encoded) => continue,
                Err(EncoderStatus::LimitReached) => break,
                Err(err) => Err(err).unwrap(),
            }
        }
        Ok(size)
    }

    pub fn write_image_repeats(&mut self, image: RgbaImage, count: usize) -> Result<usize> {
        let mut ctx = self.encode_context()?;
        let frame = ctx.new_frame();
        let mut size = 0;
        for _ in 0..count {
            ctx.send_frame(frame.clone()).unwrap();
        }
        ctx.flush();
        loop {
            match ctx.receive_packet() {
                Ok(packet) => {
                    size = self.output.write(&packet.data)?;
                    continue;
                }
                Err(EncoderStatus::Encoded) => continue,
                Err(EncoderStatus::LimitReached) => break,
                Err(err) => Err(err).unwrap(),
            }
        }
        Ok(size)
    }
    pub fn check_size(&self, width: u32, height: u32) -> Result<()> {
        if self.config.width != width as usize {
            Err(ImageError::Parameter(ParameterError::from_kind(ParameterErrorKind::DimensionMismatch)))?
        }
        if self.config.height != height as usize {
            Err(ImageError::Parameter(ParameterError::from_kind(ParameterErrorKind::DimensionMismatch)))?
        }
        Ok(())
    }

    fn encode_context(&self) -> Result<Context<u8>> {
        if self.config.chroma_sampling != ChromaSampling::Cs444 {
            panic!("Only 444 chroma sampling is supported")
        }
        let config = Config::new().with_encoder_config(self.config.clone()).with_rate_control(self.rate_control.clone());
        match config.new_context::<u8>() {
            Ok(o) => Ok(o),
            Err(e) => {
                panic!("Error creating context: {}", e)
            }
        }
    }
    fn build_rgba8_frame(&self, image: &RgbaImage, frame: &mut Frame<u8>) -> Result<()> {
        let width = self.config.width;
        let height = self.config.height;
        let mut f = frame.planes.iter_mut();
        let mut planes = image.pixels();

        // it doesn't seem to be necessary to fill padding area
        let mut y = f.next().unwrap().mut_slice(Default::default());
        let mut u = f.next().unwrap().mut_slice(Default::default());
        let mut v = f.next().unwrap().mut_slice(Default::default());

        for ((y, u), v) in y.rows_iter_mut().zip(u.rows_iter_mut()).zip(v.rows_iter_mut()).take(height) {
            let y = &mut y[..width];
            let u = &mut u[..width];
            let v = &mut v[..width];
            for ((y, u), v) in y.iter_mut().zip(u).zip(v) {
                let px = planes.next().expect("Too few pixels");
                let px = rgba8_to_yuv(px);
                *y = px[0];
                *u = px[1];
                *v = px[2];
            }
        }
        Ok(())
    }
    fn build_rgba32_frame(&self, image: &Rgba32FImage, frame: &mut Frame<u8>) -> Result<()> {
        let width = self.config.width;
        let height = self.config.height;
        let mut f = frame.planes.iter_mut();
        let mut planes = image.pixels();

        // it doesn't seem to be necessary to fill padding area
        let mut y = f.next().unwrap().mut_slice(Default::default());
        let mut u = f.next().unwrap().mut_slice(Default::default());
        let mut v = f.next().unwrap().mut_slice(Default::default());

        for ((y, u), v) in y.rows_iter_mut().zip(u.rows_iter_mut()).zip(v.rows_iter_mut()).take(height) {
            let y = &mut y[..width];
            let u = &mut u[..width];
            let v = &mut v[..width];
            for ((y, u), v) in y.iter_mut().zip(u).zip(v) {
                let px = planes.next().expect("Too few pixels");
                let px = rgba32_to_yuv(px);
                *y = px[0];
                *u = px[1];
                *v = px[2];
            }
        }
        Ok(())
    }
}

// ## RGB to YUV
// Y = (( 66 * R + 129 * G +  25 * B + 128) >> 8) +  16
// U = ((-38 * R -  74 * G + 112 * B + 128) >> 8) + 128
// V = ((112 * R -  94 * G -  18 * B + 128) >> 8) + 128
fn rgba8_to_yuv(rgba: &Rgba<u8>) -> [u8; 3] {
    let r = rgba[0] as i32;
    let g = rgba[1] as i32;
    let b = rgba[2] as i32;
    let a = rgba[3] as u8;
    let y = ((66 * r + 129 * g + 25 * b + 128) >> 8) + 16;
    let u = ((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128;
    let v = ((112 * r - 94 * g - 18 * b + 128) >> 8) + 128;
    alpha_blend_yuv([y as u8, u as u8, v as u8], a)
}

// Y =  0.299R + 0.587G + 0.114B
// U = -0.147R - 0.289G + 0.436B
// V =  0.615R - 0.515G - 0.100B
fn rgba32_to_yuv(rgba: &Rgba<f32>) -> [u8; 3] {
    let r = rgba[0];
    let g = rgba[1];
    let b = rgba[2];
    let a = rgba[3] * 255.0;
    let y = (0.299 * r + 0.587 * g + 0.114 * b) * 255.0;
    let u = (-0.147 * r - 0.289 * g + 0.436 * b) * 255.0 + 128.0;
    let v = (0.615 * r - 0.515 * g - 0.100 * b) * 255.0 + 128.0;
    alpha_blend_yuv([y as u8, u as u8, v as u8], a as u8)
}

fn alpha_blend_yuv(yuv: [u8; 3], alpha: u8) -> [u8; 3] {
    let y = yuv[0] as i32;
    let u = yuv[1] as i32;
    let v = yuv[2] as i32;
    let a = alpha as i32;
    let y = (y * a + 128) >> 8;
    let u = (u * a + 128) >> 8;
    let v = (v * a + 128) >> 8;
    [y as u8, u as u8, v as u8]
}
