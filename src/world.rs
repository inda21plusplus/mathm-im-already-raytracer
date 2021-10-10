use std::f32;

use rayon::prelude::*;
use vek::Lerp;

use crate::{
    shapes::{Intersect, Shape},
    Camera, Image, Ray, Vec4,
};

pub struct World {
    pub camera: Camera,
    pub shapes: Vec<Shape>,
}

impl World {
    pub fn render(&self, width: usize, heigth: usize) -> Image {
        let mut buffer = vec![Vec4::zero(); width * heigth];
        self.camera
            .rays(width, heigth)
            .collect::<Vec<(Ray, usize, usize)>>()
            .par_iter()
            .map(|&(ray, _, _)| self.ray_color(ray, 4))
            .collect_into_vec(&mut buffer);
        Image::new(buffer, width, heigth)
    }
    fn ray_color(&self, ray: Ray, depth: usize) -> Vec4 {
        let mut color = Vec4::zero();
        let mut min_dist = f32::MAX;
        for shape in &self.shapes {
            if let Some(intersection) = shape.intersection(ray) {
                if intersection.dist < min_dist {
                    if depth > 0 && shape.material.specularity > 0. {
                        color = Lerp::lerp(
                            shape.material.color,
                            self.ray_color(intersection.reflection(), depth - 1),
                            shape.material.specularity,
                        );
                    } else {
                        color = shape.material.color;
                    }
                    min_dist = intersection.dist;
                }
            }
        }
        color
    }
}
