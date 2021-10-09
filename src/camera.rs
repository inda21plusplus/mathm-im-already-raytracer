use vek::Lerp;

use crate::{Quaternion, Ray, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub orientation: Quaternion,
    pub fov: f32,
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
        let h_fov = v_fov * self.width as f32 / self.heigth as f32;

        let x = self.current % self.width;
        let y = self.current / self.width;

        // let yaw = (x as f32 / (self.width - 1) as f32).lerp(-h_fov / 2., h_fov / 2.);
        // let pitch = (y as f32 / (self.heigth - 1) as f32).lerp(v_fov / 2., -v_fov / 2.);

        let top = Lerp::lerp(
            Vec3::new(-1. / 3f32.sqrt(), 1. / 3f32.sqrt(), 1. / 3f32.sqrt()),
            Vec3::new(1. / 3f32.sqrt(), 1. / 3f32.sqrt(), 1. / 3f32.sqrt()),
            x as f32 / (self.width - 1) as f32,
        );
        let bot = Lerp::lerp(
            Vec3::new(-1. / 3f32.sqrt(), -1. / 3f32.sqrt(), 1. / 3f32.sqrt()),
            Vec3::new(1. / 3f32.sqrt(), -1. / 3f32.sqrt(), 1. / 3f32.sqrt()),
            x as f32 / (self.width - 1) as f32,
        );
        let direction = Lerp::lerp(top, bot, y as f32 / (self.heigth - 1) as f32).normalized();

        self.current += 1;

        Some((Ray::new(self.camera.position, direction), x, y))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.width * self.heigth;
        (size, Some(size))
    }
}
