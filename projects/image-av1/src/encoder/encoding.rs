use std::io::Write;

use image::{DynamicImage, ImageError, RgbaImage};
use rav1e::prelude::*;

use crate::Av1Encoder;

impl Av1Encoder {
    pub fn resize_image(&mut self, image: DynamicImage) -> Result<RgbaImage, ImageError> {
        image.to_rgba8().resize_exact(width, height, image::imageops::FilterType::Nearest).into_result()
    }

    pub fn write_image(&mut self, image: RgbaImage) -> Result<usize, ImageError> {
        let mut ctx = self.encode_context()?;
        let frame = ctx.new_frame();
        let mut size = 0;
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
    pub fn write_image_repeats(&mut self, image: RgbaImage, count: usize) -> Result<usize, ImageError> {
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

    fn encode_context(&self) -> Result<Context<u8>, ImageError> {
        let config = Config::new().with_encoder_config(self.encoder.clone()).with_rate_control(self.rate_control.clone());
        match config.new_context::<u8>() {
            Ok(o) => Ok(o),
            Err(e) => {
                panic!("Error creating context: {}", e)
            }
        }
    }
    fn init_frame_3(width: usize, height: usize, planes: impl IntoIterator<Item = [u8; 3]> + Send, frame: &mut Frame<u8>) -> Result<(), Error> {
        let mut f = frame.planes.iter_mut();
        let mut planes = planes.into_iter();

        // it doesn't seem to be necessary to fill padding area
        let mut y = f.next().unwrap().mut_slice(Default::default());
        let mut u = f.next().unwrap().mut_slice(Default::default());
        let mut v = f.next().unwrap().mut_slice(Default::default());

        for ((y, u), v) in y.rows_iter_mut().zip(u.rows_iter_mut()).zip(v.rows_iter_mut()).take(height) {
            let y = &mut y[..width];
            let u = &mut u[..width];
            let v = &mut v[..width];
            for ((y, u), v) in y.iter_mut().zip(u).zip(v) {
                let px = planes.next().ok_or(Error::TooFewPixels)?;
                *y = px[0];
                *u = px[1];
                *v = px[2];
            }
        }
        Ok(())
    }

    fn init_frame_1(width: usize, height: usize, planes: impl IntoIterator<Item = u8> + Send, frame: &mut Frame<u8>) -> Result<(), Error> {
        let mut y = frame.planes[0].mut_slice(Default::default());
        let mut planes = planes.into_iter();

        for y in y.rows_iter_mut().take(height) {
            let y = &mut y[..width];
            for y in y.iter_mut() {
                *y = planes.next().ok_or(Error::TooFewPixels)?;
            }
        }
        Ok(())
    }
}
