use std::f32;

use rayon::prelude::*;
use vek::{Lerp, Quaternion};

use crate::{
    shapes::{Intersect, Shape},
    Camera, Image, Ray, Vec3,
};

pub struct World {
    pub camera: Camera,
    pub shapes: Vec<Shape>,
}

impl World {
    pub fn render(&self, width: usize, heigth: usize) -> Image {
        let mut buffer = vec![Vec3::zero(); width * heigth];
        self.camera
            .rays(width, heigth)
            .collect::<Vec<(Ray, usize, usize)>>()
            .par_iter()
            .map(|&(ray, _, _)| self.ray_color(ray, 1))
            .collect_into_vec(&mut buffer);
        Image::new(buffer, width, heigth)
    }
    fn ray_color(&self, ray: Ray, depth: usize) -> Vec3 {
        let mut color = Vec3::zero();
        let mut min_dist = f32::MAX;
        for shape in &self.shapes {
            if let Some(intersection) = shape.intersection(ray) {
                if intersection.dist < min_dist {
                    if depth == 0 {
                        color = shape.material.color;
                    } else {
                        let reflected = Ray::new(
                            intersection.point,
                            Quaternion::rotation_3d(180f32.to_radians(), intersection.normal)
                                * -ray.direction,
                        );
                        color = Lerp::lerp(
                            shape.material.color,
                            self.ray_color(reflected, depth - 1),
                            shape.material.specularity,
                        );
                    }
                    min_dist = intersection.dist;
                }
            }
        }
        color
    }
}
