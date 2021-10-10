use std::{fs::File, time::Instant};

use im_already_raytracer::{presets, render, Error as IARTError};
use png::EncodingError;

#[derive(Debug)]
pub enum Error {
    RayTracerError(IARTError),
    EncodingError(EncodingError),
}

impl From<IARTError> for Error {
    fn from(err: IARTError) -> Self {
        Self::RayTracerError(err)
    }
}

impl From<EncodingError> for Error {
    fn from(err: EncodingError) -> Self {
        Self::EncodingError(err)
    }
}

fn main() -> Result<(), Error> {
    let (camera, shapes) = presets::cornellbox();

    let start = Instant::now();
    let image = render(&camera, &shapes, 1000, 1000);
    let dur = start.elapsed();
    println!("{}ms", dur.as_millis());
    let data = image.get_raw_data();

    let file = File::create("output.png").unwrap();
    let mut encoder = png::Encoder::new(file, image.width as u32, image.heigth as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.write_header()?.write_image_data(&data)?;

    Ok(())
}
