use std::f32;

use rayon::prelude::*;
use vek::Lerp;

use crate::{
    shapes::{Intersect, Shape},
    Camera, Image, Ray, Vec3,
};

pub fn render(camera: &Camera, shapes: &[Shape], width: usize, height: usize) -> Image {
    let mut buffer = vec![Vec3::zero(); width * height];
    camera
        .rays(width, height)
        .collect::<Vec<(Ray, usize, usize)>>()
        .par_iter()
        .map(|&(ray, _, _)| ray_color(ray, shapes, 10, None))
        .collect_into_vec(&mut buffer);
    Image::new(buffer, width, height)
}

fn ray_color(ray: Ray, shapes: &[Shape], depth: usize, ignore_normal: Option<Vec3>) -> Vec3 {
    let mut color = Vec3::zero();
    let mut min_dist = f32::MAX;
    for shape in shapes {
        let mat = &shape.material;
        let intersection = match shape.intersection(ray, ignore_normal) {
            Some(i) if i.dist < min_dist => i,
            _ => continue,
        };
        if depth == 0 {
            color = mat.color;
            min_dist = intersection.dist;
            continue;
        }

        let reflection_color = if mat.specularity > 0. {
            ray_color(
                intersection.reflection(mat.roughness),
                shapes,
                depth - 1,
                Some(intersection.normal),
            )
        } else {
            Vec3::zero()
        };
        let refraction_color = if mat.opacity < 1. {
            ray_color(
                intersection.refraction(mat.refractive_index),
                shapes,
                depth - 1,
                Some(intersection.normal),
            )
        } else {
            Vec3::zero()
        };

        color = Lerp::lerp(
            Lerp::lerp(refraction_color, mat.color, mat.opacity),
            reflection_color,
            mat.specularity,
        );

        min_dist = intersection.dist;
    }
    color
}
