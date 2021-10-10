use std::f32;

use rayon::prelude::*;
use vek::Lerp;

use crate::{
    shapes::{Intersect, Shape},
    Camera, Image, Ray, Vec4,
};

pub fn render(camera: &Camera, shapes: &[Shape], width: usize, heigth: usize) -> Image {
    let mut buffer = vec![Vec4::zero(); width * heigth];
    camera
        .rays(width, heigth)
        .collect::<Vec<(Ray, usize, usize)>>()
        .par_iter()
        .map(|&(ray, _, _)| ray_color(ray, shapes, 4))
        .collect_into_vec(&mut buffer);
    Image::new(buffer, width, heigth)
}

fn ray_color(ray: Ray, shapes: &[Shape], depth: usize) -> Vec4 {
    let mut color = Vec4::zero();
    let mut min_dist = f32::MAX;
    for shape in shapes {
        if let Some(intersection) = shape.intersection(ray) {
            if intersection.dist < min_dist {
                if depth > 0 && shape.material.specularity > 0. {
                    color = Lerp::lerp(
                        shape.material.color,
                        ray_color(intersection.reflection(), shapes, depth - 1),
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
