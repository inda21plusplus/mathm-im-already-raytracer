use std::arch::x86_64;

use crate::{f32x8, i32x8, splatf32x8, Quaternion, Ray, Vec3};

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

fn lerp_simd(from: f32x8, to: f32x8, t: f32x8) -> f32x8 {
    unsafe {
        x86_64::_mm256_add_ps(
            x86_64::_mm256_mul_ps(from, x86_64::_mm256_sub_ps(splatf32x8(1.), t)),
            x86_64::_mm256_mul_ps(to, t),
        )
    }
}

impl MappingFunction {
    pub fn get_direction(&self, x01: f32, y01: f32, aspect_ratio: f32, fov: f32) -> Vec3 {
        match self {
            _ => Self::linear(x01, y01, aspect_ratio, fov),
            // Self::Unlinear => Self::unlinear(x01, y01, aspect_ratio, fov),
            // Self::Circular => Self::circular(x01, y01, aspect_ratio),
        }
    }
    fn linear(x01: f32, y01: f32, aspect_ratio: f32, fov: f32) -> Vec3 {
        Vec3 {
            x: lerp(-1., 1., x01) * (fov / 2.).tan() * aspect_ratio,
            y: lerp(-1., 1., y01) * (fov / 2.).tan(),
            z: -1.,
        }
        .normalized()
    }
    pub fn linear_simd(
        x01s: f32x8,
        y01: f32,
        aspect_ratio: f32,
        fov: f32,
    ) -> (f32x8, f32x8, f32x8) {
        unsafe {
            let xs = x86_64::_mm256_mul_ps(
                lerp_simd(splatf32x8(-1.), splatf32x8(1.), x01s),
                splatf32x8((fov / 2.).tan() * aspect_ratio),
            );
            let ys = splatf32x8(lerp(-1., 1., y01) * (fov / 2.).tan());
            let zs = splatf32x8(-1.);
            let lensq = x86_64::_mm256_add_ps(
                x86_64::_mm256_add_ps(x86_64::_mm256_mul_ps(xs, xs), x86_64::_mm256_mul_ps(ys, ys)),
                zs, // == zs * zs
            );
            let rlen = x86_64::_mm256_rsqrt_ps(lensq);
            let xs = x86_64::_mm256_mul_ps(xs, rlen);
            let ys = x86_64::_mm256_mul_ps(ys, rlen);
            let zs = x86_64::_mm256_mul_ps(zs, rlen);
            (xs, ys, zs)
        }
    }
    fn unlinear(x01: f32, y01: f32, aspect_ratio: f32, fov: f32) -> Vec3 {
        Vec3 {
            x: (lerp(-1., 1., x01) * fov / 2.).tan() * aspect_ratio,
            y: (lerp(-1., 1., y01) * fov / 2.).tan(),
            z: -1.,
        }
    }
    fn circular(x01: f32, y01: f32, aspect_ratio: f32) -> Vec3 {
        let x = lerp(-1., 1., x01) * aspect_ratio;
        let y = lerp(-1., 1., y01);
        Vec3 {
            x,
            y,
            z: -(1. - x * x - y * y).sqrt(),
        }
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
        let aspect_ratio = self.width as f32 / self.height as f32;

        let x01 = x as f32 / (self.width - 1) as f32;
        let y01 = 1. - (y as f32 / (self.height - 1) as f32);

        let direction =
            self.camera
                .mapping_function
                .get_direction(x01, y01, aspect_ratio, self.camera.fov);

        Ray::new(self.camera.position, self.camera.orientation * direction)
    }
    pub fn get_simd(&self, xs: i32x8, y: usize) -> (Vec3, f32x8, f32x8, f32x8) {
        unsafe {
            let aspect_ratio = self.width as f32 / self.height as f32;
            let xsf = x86_64::_mm256_cvtepi32_ps(xs);
            let x01s = x86_64::_mm256_div_ps(xsf, splatf32x8((self.width - 1) as f32));
            let y01 = 1. - (y as f32 / (self.height - 1) as f32);

            let (dxs, dys, dzs) =
                MappingFunction::linear_simd(x01s, y01, aspect_ratio, self.camera.fov);

            // self.camera.orientation * direction(=dxs, dys, dzs):

            let q = self.camera.orientation;
            let qi = q.conjugate();

            // q.w*d + 0*q.xyz + q × d
            let qd_xs = x86_64::_mm256_add_ps(
                x86_64::_mm256_mul_ps(dxs, splatf32x8(q.w)),
                // q × d
                x86_64::_mm256_sub_ps(
                    x86_64::_mm256_mul_ps(splatf32x8(q.y), dzs),
                    x86_64::_mm256_mul_ps(splatf32x8(q.z), dys),
                ),
            );
            let qd_ys = x86_64::_mm256_add_ps(
                x86_64::_mm256_mul_ps(dxs, splatf32x8(q.w)),
                // q × d
                x86_64::_mm256_sub_ps(
                    x86_64::_mm256_mul_ps(splatf32x8(q.z), dxs),
                    x86_64::_mm256_mul_ps(splatf32x8(q.x), dzs),
                ),
            );
            let qd_zs = x86_64::_mm256_add_ps(
                x86_64::_mm256_mul_ps(dxs, splatf32x8(q.w)),
                // q × d
                x86_64::_mm256_sub_ps(
                    x86_64::_mm256_mul_ps(splatf32x8(q.x), dys),
                    x86_64::_mm256_mul_ps(splatf32x8(q.y), dxs),
                ),
            );

            // q.w*0 - q.xyz `dot` d
            let qd_ws = x86_64::_mm256_sub_ps(
                splatf32x8(0.),
                x86_64::_mm256_add_ps(
                    x86_64::_mm256_add_ps(
                        x86_64::_mm256_mul_ps(splatf32x8(q.x), dxs),
                        x86_64::_mm256_mul_ps(splatf32x8(q.y), dys),
                    ),
                    x86_64::_mm256_mul_ps(splatf32x8(q.z), dzs),
                ),
            );

            // qd.w * qi.xyz + qi.w * qd.xyz + qd.xyz × qi.xyz
            let qdqi_xs = x86_64::_mm256_add_ps(
                x86_64::_mm256_add_ps(
                    x86_64::_mm256_mul_ps(qd_ws, splatf32x8(qi.x)),
                    x86_64::_mm256_mul_ps(splatf32x8(qi.w), qd_xs),
                ),
                // q × d
                x86_64::_mm256_sub_ps(
                    x86_64::_mm256_mul_ps(qd_ys, splatf32x8(qi.z)),
                    x86_64::_mm256_mul_ps(qd_zs, splatf32x8(qi.y)),
                ),
            );
            let qdqi_ys = x86_64::_mm256_add_ps(
                x86_64::_mm256_add_ps(
                    x86_64::_mm256_mul_ps(qd_ws, splatf32x8(qi.y)),
                    x86_64::_mm256_mul_ps(splatf32x8(qi.w), qd_ys),
                ),
                // q × d
                x86_64::_mm256_sub_ps(
                    x86_64::_mm256_mul_ps(qd_zs, splatf32x8(qi.x)),
                    x86_64::_mm256_mul_ps(qd_xs, splatf32x8(qi.z)),
                ),
            );
            let qdqi_zs = x86_64::_mm256_add_ps(
                x86_64::_mm256_add_ps(
                    x86_64::_mm256_mul_ps(qd_ws, splatf32x8(qi.z)),
                    x86_64::_mm256_mul_ps(splatf32x8(qi.w), qd_zs),
                ),
                // q × d
                x86_64::_mm256_sub_ps(
                    x86_64::_mm256_mul_ps(qd_xs, splatf32x8(qi.y)),
                    x86_64::_mm256_mul_ps(qd_ys, splatf32x8(qi.x)),
                ),
            );

            (self.camera.position, qdqi_xs, qdqi_ys, qdqi_zs)
        }
    }
}
