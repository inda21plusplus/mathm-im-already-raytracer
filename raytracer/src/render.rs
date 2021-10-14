use std::f32;

use itertools::Itertools;
use rayon::prelude::*;
use vek::Lerp;

use crate::{
    shapes::{Intersect, Intersection, Shape},
    Camera, Image, Light, Ray, Vec3,
};

pub struct RenderOptions {
    pub multisampling: usize,
    pub width: usize,
    pub height: usize,
    pub max_ray_depth: usize,
    pub soft_shadow_resolution: usize,
    pub use_randomness: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            multisampling: 1,
            width: 640,
            height: 640,
            max_ray_depth: 5,
            soft_shadow_resolution: 4,
            use_randomness: true,
        }
    }
}

pub fn render(
    options: &RenderOptions,
    camera: &Camera,
    shapes: &[Shape],
    lights: &[Light],
) -> Image {
    let upsampled_width = options.width * options.multisampling;
    let upsampled_height = options.height * options.multisampling;
    let mut buffer = vec![Vec3::zero(); upsampled_width * upsampled_height];
    camera
        .rays(upsampled_width, upsampled_height)
        .collect::<Vec<(Ray, usize, usize)>>()
        .par_iter()
        .map(|&(ray, _, _)| ray_color(options, ray, shapes, lights, 0, None))
        .collect_into_vec(&mut buffer);
    let buffer = downsample(options, &buffer);
    Image::new(buffer, options.width, options.height)
}

fn downsample(options: &RenderOptions, buffer: &[Vec3]) -> Vec<Vec3> {
    let original_width = options.multisampling * options.width;

    (0..options.height)
        .cartesian_product(0..options.width)
        .map(|(row, col)| {
            let mut color = Vec3::zero();
            for r in 0..options.multisampling {
                for c in 0..options.multisampling {
                    let i = (row * options.multisampling + r) * original_width
                        + col * options.multisampling
                        + c;
                    color += buffer[i];
                }
            }
            color / (options.multisampling * options.multisampling) as f32
        })
        .collect()
}

fn ray_intersection<'s, Intersectable>(
    ray: Ray,
    intersectables: impl Iterator<Item = &'s Intersectable>,
    ignore_normal: Option<Vec3>,
) -> Option<(&'s Intersectable, Intersection)>
where
    Intersectable: Intersect,
{
    let mut min_dist = f32::MAX;
    let mut closest: Option<(&Intersectable, Intersection)> = None;
    for intersectable in intersectables {
        let intersection = match intersectable.intersection(ray, ignore_normal) {
            Some(i) if i.dist < min_dist => i,
            _ => continue,
        };

        min_dist = intersection.dist;
        closest = Some((intersectable, intersection));
    }

    return closest;
}

fn ray_color(
    options: &RenderOptions,
    ray: Ray,
    shapes: &[Shape],
    lights: &[Light],
    depth: usize,
    ignore_normal: Option<Vec3>,
) -> Vec3 {
    if depth == options.max_ray_depth {
        return Vec3::zero(); // todo: something better
    }

    let (shape, intersection) = match ray_intersection(ray, shapes.iter(), ignore_normal) {
        Some(shape_intersection) => shape_intersection,
        None => return Vec3::zero(), // todo: skybox
    };

    if let Some((light, light_intersection)) = ray_intersection(ray, lights.iter(), None) {
        // TODO: what if theyre equal? maybe check normal?
        if light_intersection.dist < intersection.dist {
            return Vec3::broadcast(light.intensity);
        }
    };

    let mat = &shape.material;

    let reflection_color = if mat.specularity > 0. {
        ray_color(
            options,
            intersection.reflection(mat.roughness, options.use_randomness),
            shapes,
            lights,
            depth + 1,
            Some(intersection.normal),
        )
    } else {
        Vec3::zero()
    };
    let refraction_color = if mat.opacity < 1. {
        ray_color(
            options,
            intersection.refraction(mat.refractive_index),
            shapes,
            lights,
            depth + 1,
            Some(intersection.normal),
        )
    } else {
        Vec3::zero()
    };

    let mut lambert = 0.;
    for light in lights {
        let rays = light.rays_to(intersection.point, options.soft_shadow_resolution);
        let ray_count = rays.len();
        let mut hits = 0;
        for r in rays {
            if let Some((s, _)) = ray_intersection(r, shapes.iter(), None) {
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
            hits += 1;
        }
        let hit_factor = if ray_count > 0 {
            hits as f32 / ray_count as f32
        } else {
            1.
        };
        lambert += light.lambert(intersection.point, intersection.normal) * hit_factor;
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
