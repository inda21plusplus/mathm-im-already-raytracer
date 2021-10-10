use crate::{Quaternion, Ray, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub orientation: Quaternion,
    pub fov: f32,
    pub mapping_function: MappingFunction,
}

pub enum MappingFunction {
    Linear,
    Unlinear,
    Circular,
}

impl MappingFunction {
    pub fn get_direction(&self, x01: f32, y01: f32, fov: f32, aspect_ratio: f32) -> Vec3 {
        match self {
            Self::Linear => Vec3 {
                x: x01.lerp(-1., 1.) * (fov / 2.).tan() * aspect_ratio,
                y: y01.lerp(-1., 1.) * (fov / 2.).tan(),
                z: 1.,
            },
            Self::Unlinear => Vec3 {
                x: (x01.lerp(-1., 1.) * fov / 2.).tan() * aspect_ratio,
                y: (y01.lerp(-1., 1.) * fov / 2.).tan(),
                z: 1.,
            },
            Self::Circular => {
                let x = x01.lerp(-1., 1.) * aspect_ratio;
                let y = y01.lerp(-1., 1.);
                Vec3 {
                    x,
                    y,
                    z: (1. - x * x - y * y).sqrt(),
                }
            }
        }
        .normalized()
    }
}

impl Camera {
    pub fn rays(&self, width: usize, heigth: usize) -> Rays {
        Rays {
            camera: self,
            width,
            heigth,
            current: 0,
        }
    }
}

pub struct Rays<'c> {
    camera: &'c Camera,
    width: usize,
    heigth: usize,
    current: usize,
}

impl<'c> Iterator for Rays<'c> {
    type Item = (Ray, usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.width * self.heigth {
            return None;
        }

        let v_fov = self.camera.fov;
        let aspect_ratio = self.width as f32 / self.heigth as f32;

        let x = self.current % self.width;
        let y = self.current / self.width;

        let x01 = x as f32 / (self.width - 1) as f32;
        let y01 = 1. - (y as f32 / (self.heigth - 1) as f32);

        let direction = self
            .camera
            .mapping_function
            .get_direction(x01, y01, v_fov, aspect_ratio);

        self.current += 1;

        Some((
            Ray::new(self.camera.position, self.camera.orientation * direction),
            x,
            y,
        ))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.width * self.heigth;
        (size, Some(size))
    }
}
