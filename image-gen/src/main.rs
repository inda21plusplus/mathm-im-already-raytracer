use std::{
    fs::File,
    io::{stdout, Error as IOError, Write},
    time::Instant,
};

use im_already_raytracer::{presets, render, Error as IARTError};
use png::EncodingError;

#[derive(Debug)]
pub enum Error {
    RayTracerError(IARTError),
    EncodingError(EncodingError),
    IOError(IOError),
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

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}

fn main() -> Result<(), Error> {
    let (camera, shapes) = presets::cornellbox();

    let width = 1000;
    let height = 1000;
    let mut data = vec![0u8; width * height * 4];
    let start = Instant::now();
    let iterations = 10;
    for i in 1..=iterations {
        print!(
            "\r                       \r[{} / {}] {}",
            i - 1,
            iterations,
            ".".repeat(i % 4)
        );
        stdout().lock().flush()?;

        let image = render(&camera, &shapes, width, height);
        for (b, n) in data.iter_mut().zip(image.get_raw_data()) {
            let f_o = (i as f32 - 1.) / i as f32;
            let f_n = 1. / i as f32;
            *b = (*b as f32 * f_o + n as f32 * f_n) as u8;
        }

        let file = File::create("output.png").unwrap();
        let mut encoder = png::Encoder::new(file, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.write_header()?.write_image_data(&data)?;
    }
    let dur = start.elapsed();
    println!("\r{:.2} s         ", dur.as_secs_f32());

    Ok(())
}
