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

fn lerp(from: f32, to: f32, t: f32) -> f32 {
    from * (1. - t) + to * t
}

impl MappingFunction {
    pub fn get_direction(&self, x01: f32, y01: f32, fov: f32, aspect_ratio: f32) -> Vec3 {
        match self {
            Self::Linear => Vec3 {
                x: lerp(-1., 1., x01) * (fov / 2.).tan() * aspect_ratio,
                y: lerp(-1., 1., y01) * (fov / 2.).tan(),
                z: -1.,
            },
            Self::Unlinear => Vec3 {
                x: (lerp(-1., 1., x01) * fov / 2.).tan() * aspect_ratio,
                y: (lerp(-1., 1., y01) * fov / 2.).tan(),
                z: -1.,
            },
            Self::Circular => {
                let x = lerp(-1., 1., x01) * aspect_ratio;
                let y = lerp(-1., 1., y01);
                Vec3 {
                    x,
                    y,
                    z: -(1. - x * x - y * y).sqrt(),
                }
            }
        }
        .normalized()
    }
}

impl Camera {
    pub fn rays(&self, width: usize, height: usize) -> Rays {
        Rays {
            camera: self,
            width,
            height,
        }
    }
}

pub struct Rays<'c> {
    camera: &'c Camera,
    width: usize,
    height: usize,
}

impl<'c> Rays<'c> {
    pub fn get(&self, x: usize, y: usize) -> Ray {
        let v_fov = self.camera.fov;
        let aspect_ratio = self.width as f32 / self.height as f32;

        let x01 = x as f32 / (self.width - 1) as f32;
        let y01 = 1. - (y as f32 / (self.height - 1) as f32);

        let direction = self
            .camera
            .mapping_function
            .get_direction(x01, y01, v_fov, aspect_ratio);

        Ray::new(self.camera.position, self.camera.orientation * direction)
    }
}
