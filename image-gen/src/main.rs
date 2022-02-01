use std::{fs::File, io::Error as IOError};

use im_already_raytracer::{
    presets,
    render::{render, RenderOptions},
    Error as IARTError,
};
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
    let (camera, shapes, lights) = presets::cornellbox();

    let options = RenderOptions {
        width: 1000,
        height: 1000,
        multisampling: 12,
        soft_shadow_resolution: 8,
        ..Default::default()
    };
    let image = render(&options, &camera, &shapes, &lights);

    let file = File::create("output.png").unwrap();
    let mut encoder = png::Encoder::new(file, options.width as u32, options.height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()?
        .write_image_data(&image.get_raw_data())?;

    Ok(())
}
