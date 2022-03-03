use std::ops::Range;

use vek::Lerp;

use crate::{
    camera::Rays,
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
    pub clamp_colors: bool,
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
            clamp_colors: true,
        }
    }
}

pub fn render(
    options: &RenderOptions,
    camera: &Camera,
    shapes: &[Shape],
    lights: &[Light],
) -> Image {
    let mut buffer = vec![Vec3::zero(); options.width * options.height];
    let rays = camera.rays(
        options.width * options.multisampling,
        options.height * options.multisampling,
    );
    #[cfg(feature = "parallel")]
    {
        crossbeam::scope(|s| {
            for (range, output) in split_buffer(&mut buffer, 12) {
                s.spawn(|_| render_part(options, range, output, &rays, shapes, lights));
            }
        })
        .unwrap();
    }
    #[cfg(not(feature = "parallel"))]
    {
        render_part(options, 0..buffer.len(), &mut buffer, &rays, shapes, lights);
    }
    Image::new(buffer, options.width, options.height)
}

fn render_part(
    options: &RenderOptions,
    range: Range<usize>,
    output: &mut [Vec3],
    rays: &Rays,
    shapes: &[Shape],
    lights: &[Light],
) {
    let ms = options.multisampling;
    for (out_i, pos_i) in range.enumerate() {
        let base_x = pos_i % options.width;
        let base_y = pos_i / options.width;
        let mut color_sum = Vec3::zero();
        for y in (0..ms).map(|s| base_y * ms + s) {
            for x in (0..ms).map(|s| base_x * ms + s) {
                let color = ray_color(options, rays.get(x, y), shapes, lights, 0, None);
                color_sum += if options.clamp_colors {
                    clamp_color(color)
                } else {
                    color
                };
            }
        }
        let pixel_color = color_sum / (ms * ms) as f32;
        output[out_i] = pixel_color;
    }
}

#[rustfmt::skip]
fn clamp_color(color: Vec3) -> Vec3 {
    Vec3::new(
        if color.x < 0. { 0. } else if color.x > 1. { 1. } else { color.x },
        if color.y < 0. { 0. } else if color.y > 1. { 1. } else { color.y },
        if color.z < 0. { 0. } else if color.z > 1. { 1. } else { color.z },
    )
}

fn split_buffer<'a, T>(mut buffer: &'a mut [T], parts: usize) -> Vec<(Range<usize>, &'a mut [T])> {
    let mut v = Vec::with_capacity(parts);
    let orig_len = buffer.len();
    let mut start = 0;
    for p in (1..=parts).rev() {
        let (a, b) = buffer.split_at_mut(buffer.len() / p);
        buffer = b;
        let end = start + a.len();
        v.push((start..end, a));
        start = end;
    }
    assert_eq!(v[0].0.start, 0);
    for i in 1..parts {
        assert_eq!(v[i - 1].0.end, v[i].0.start, "{}", i);
    }
    assert_eq!(v[parts - 1].0.end, orig_len);
    assert_eq!(buffer.len(), 0);
    v
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
