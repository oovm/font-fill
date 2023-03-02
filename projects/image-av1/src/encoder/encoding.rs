use crate::Av1Encoder;
use image::RgbaImage;
use rav1e::prelude::*;

impl Av1Encoder {
    pub fn encode_frame(&mut self, frame: RgbaImage) -> Result<(), ImageError> {
        let cfg = Config::default();
        let mut ctx: Context<u8> = cfg.new_context()?;
        let frame = ctx.new_frame();

        ctx.send_frame(frame)?;
        ctx.flush();

        loop {
            match ctx.receive_packet() {
                Ok(packet) => { /* Mux the packet. */ }
                Err(EncoderStatus::Encoded) => (),
                Err(EncoderStatus::LimitReached) => break,
                Err(err) => Err(err)?,
            }
        }
    }
}
