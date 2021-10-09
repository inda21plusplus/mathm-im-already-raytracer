use std::io::Write;

use crate::{Error, Vec3};

pub struct Image {
    data: Vec<Vec3>,
    width: usize,
    heigth: usize,
}

impl Image {
    pub fn new(data: Vec<Vec3>, width: usize, heigth: usize) -> Self {
        assert_eq!(data.len(), width * heigth);
        Self {
            data,
            width,
            heigth,
        }
    }
    pub fn write(&self, w: impl Write) -> Result<(), Error> {
        let mut encoder = png::Encoder::new(w, self.width as u32, self.heigth as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::new(1. / 2.2));
        let mut writer = encoder.write_header()?;
        let mut s_writer = writer.stream_writer()?;
        for v in self.data.iter() {
            let r = (v.x * 255.).clamp(0., 255.) as u8;
            let g = (v.y * 255.).clamp(0., 255.) as u8;
            let b = (v.z * 255.).clamp(0., 255.) as u8;
            s_writer.write(&[r, g, b])?;
        }
        Ok(())
    }
}
