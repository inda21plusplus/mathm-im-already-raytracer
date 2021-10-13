use std::f32;

use rayon::prelude::*;
use vek::Lerp;

use crate::{
    shapes::{Intersect, Intersection, Shape},
    Camera, Image, Light, Ray, Vec3,
};

pub fn render(
    camera: &Camera,
    shapes: &[Shape],
    lights: &[Light],
    width: usize,
    height: usize,
) -> Image {
    let mut buffer = vec![Vec3::zero(); width * height];
    camera
        .rays(width, height)
        .collect::<Vec<(Ray, usize, usize)>>()
        .par_iter()
        .map(|&(ray, _, _)| ray_color(ray, shapes, lights, 10, None))
        .collect_into_vec(&mut buffer);
    Image::new(buffer, width, height)
}

fn ray_intersection(
    ray: Ray,
    shapes: &[Shape],
    ignore_normal: Option<Vec3>,
) -> Option<(&Shape, Intersection)> {
    let mut min_dist = f32::MAX;
    let mut closest: Option<(&Shape, Intersection)> = None;
    for shape in shapes {
        let intersection = match shape.intersection(ray, ignore_normal) {
            Some(i) if i.dist < min_dist => i,
            _ => continue,
        };

        min_dist = intersection.dist;
        closest = Some((shape, intersection));
    }

    return closest;
}

fn ray_color(
    ray: Ray,
    shapes: &[Shape],
    lights: &[Light],
    depth: usize,
    ignore_normal: Option<Vec3>,
) -> Vec3 {
    if depth == 0 {
        return Vec3::zero(); // todo: something better
    }

    let (shape, intersection) = match ray_intersection(ray, shapes, ignore_normal) {
        Some(si) => si,
        None => return Vec3::zero(), // todo: skybox
    };
    let mat = &shape.material;

    let reflection_color = if mat.specularity > 0. {
        ray_color(
            intersection.reflection(mat.roughness),
            shapes,
            lights,
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
            lights,
            depth - 1,
            Some(intersection.normal),
        )
    } else {
        Vec3::zero()
    };

    let mut lambert = 0.;
    for light in lights {
        if let Some(r) = light.ray_to(intersection.point) {
            if let Some((s, _)) = ray_intersection(r, shapes, None) {
                // todo: maybe it would be nice to compare the pointers here instead.
                if *s != *shape {
                    continue;
                }
            }
            let light_side = intersection.normal.dot(r.direction).signum();
            let watch_side = intersection.normal.dot(ray.direction).signum();
            if light_side != watch_side {
                // The light is on the other side of the object
                continue;
            }
        }
        lambert += light.lambert(intersection.point, intersection.normal);
    }

    let matt_color = mat.color * lambert;

    Lerp::lerp(
        Lerp::lerp(refraction_color, matt_color, mat.opacity),
        reflection_color,
        if ray.direction.dot(intersection.normal) < 0. {
            mat.specularity
        } else {
            0.
        },
    )
}
