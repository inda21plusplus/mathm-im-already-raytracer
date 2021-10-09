use std::f32;

use crate::{shapes, Camera, Image, Vec3};

pub struct World {
    pub camera: Camera,
    pub shapes: Vec<Box<dyn shapes::Shape>>,
}

impl World {
    pub fn render(&self, width: usize, heigth: usize) -> Image {
        let mut buffer = vec![Vec3::zero(); width * heigth];
        for (ray, x, y) in self.camera.rays(width, heigth) {
            let mut min_dist = f32::MAX;
            for shape in &self.shapes {
                if let Some(dist) = shape.intersection_dist(ray) {
                    if 0. < dist && dist < min_dist {
                        buffer[x + width * y] = shape.material().color;
                        min_dist = dist;
                    }
                }
            }
        }
        Image::new(buffer, width, heigth)
    }
}
