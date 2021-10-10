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
        .map(|&(ray, _, _)| ray_color(ray, shapes, 4))
        .collect_into_vec(&mut buffer);
    Image::new(buffer, width, height)
}

fn ray_color(ray: Ray, shapes: &[Shape], depth: usize) -> Vec3 {
    let mut color = Vec3::zero();
    let mut min_dist = f32::MAX;
    for shape in shapes {
        let mat = &shape.material;
        let intersection = match shape.intersection(ray) {
            Some(i) if i.dist < min_dist => i,
            _ => continue,
        };
        if depth == 0 {
            color = mat.color;
            min_dist = intersection.dist;
            continue;
        }
        let reflection_color = ray_color(intersection.reflection(mat.roughness), shapes, depth - 1);
        let refraction_color = ray_color(
            intersection.refraction(mat.refractive_index),
            shapes,
            depth - 1,
        );

        color = Lerp::lerp(
            Lerp::lerp(refraction_color, mat.color, mat.opacity),
            reflection_color,
            mat.specularity,
        );

        min_dist = intersection.dist;
    }
    color
}
