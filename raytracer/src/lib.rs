#![feature(option_result_contains, test)]

pub mod camera;
pub mod error;
pub mod image;
pub mod lights;
pub mod material;
pub mod presets;
pub mod render;
pub mod shapes;

pub use camera::Camera;
pub use error::Error;
pub use image::Image;
pub use lights::Light;
pub use material::Material;
pub use render::render;

pub type Vec3 = vek::vec::repr_simd::Vec3<f32>;
pub type Vec4 = vek::vec::repr_simd::Vec4<f32>;
pub type Transform = vek::transform::repr_simd::Transform<f32, f32, f32>;
pub type Ray = vek::geom::repr_simd::Ray<f32>;
pub type Quaternion = vek::quaternion::repr_simd::Quaternion<f32>;
pub type Mat3 = vek::mat::repr_simd::column_major::Mat3<f32>;
pub type Mat4 = vek::mat::repr_simd::column_major::Mat4<f32>;

pub use std::arch::x86_64::{
    __m256 as f32x8, __m256i as i32x8, _mm256_set1_epi32 as splati32x8,
    _mm256_set1_ps as splatf32x8,
};
#[derive(Clone, Copy)]
pub struct Vec3x8 {
    pub xs: f32x8,
    pub ys: f32x8,
    pub zs: f32x8,
}
impl Vec3x8 {
    pub fn new(xs: f32x8, ys: f32x8, zs: f32x8) -> Self {
        Self { xs, ys, zs }
    }
}

// generates two arbitrary vectors that orthogonal to `v` and each other, and
// normalized if `v` is
fn orthogonal(v: Vec3) -> (Vec3, Vec3) {
    let mut a = v.cross(Vec3::unit_x());
    if a.magnitude_squared() < 0.01 {
        a = v.cross(Vec3::unit_y());
    }
    a.normalize();
    let b = v.cross(a);

    (a, b)
}

fn orthogonal_smarter(v: Vec3) -> (Vec3, Vec3) {
    let mut a = Vec3::new(0., v.z, -v.y); // v × (1, 0, 0)
    if a.magnitude_squared() < 0.01 {
        a = Vec3::new(-v.z, 0., v.x); // v × (0, 1, 0)
    }
    a.normalize();
    let b = v.cross(a);

    (a, b)
}

use std::arch::x86_64;
// v = (x, y, z, 0)
fn orthogonal_simd(v: x86_64::__m128) -> (x86_64::__m128, x86_64::__m128) {
    unsafe {
        let minus_v = x86_64::_mm_sub_ps(x86_64::_mm_set1_ps(0.), v);
        //                                     xx yy zz 00
        let mut a = x86_64::_mm_shuffle_ps::<0b00_01_10_00>(minus_v, v); // v × (1, 0, 0) = (0, v.z, -v.y)
        let mut a_len_sq = x86_64::_mm_dp_ps::<0b1110_1111>(a, a);
        let a_len_sq_lt_0_01 = x86_64::_mm_cmplt_ss(a_len_sq, x86_64::_mm_set1_ps(0.01));
        let a_len_sq_lt_0_01 = x86_64::_mm_castps_si128(a_len_sq_lt_0_01);
        if x86_64::_mm_cvtsi128_si32(a_len_sq_lt_0_01) != 0 {
            //                             xx yy zz 00
            a = x86_64::_mm_shuffle_ps::<0b01_00_11_00>(v, minus_v);
            a_len_sq = x86_64::_mm_dp_ps::<0b1110_1111>(a, a);
        }
        let rsqrt = x86_64::_mm_rsqrt_ps(a_len_sq);
        let a = x86_64::_mm_mul_ps(a, rsqrt);
        let v_rot_l = x86_64::_mm_shuffle_ps::<0b10_01_11_00>(v, v);
        let v_rot_r = x86_64::_mm_shuffle_ps::<0b01_11_10_00>(v, v);
        let a_rot_l = x86_64::_mm_shuffle_ps::<0b10_01_11_00>(a, a);
        let a_rot_r = x86_64::_mm_shuffle_ps::<0b01_11_10_00>(a, a);
        let b = x86_64::_mm_sub_ps(
            x86_64::_mm_mul_ps(v_rot_l, a_rot_r),
            x86_64::_mm_mul_ps(v_rot_r, a_rot_l),
        );
        (a, b)
    }
}

fn orthogonal_simd_8(vs: Vec3x8) -> (Vec3x8, Vec3x8) {
    unsafe {
        // let mut a = Vec3::new(0., v.z, -v.y); // v × (1, 0, 0)
        let mut axs = splatf32x8(0.);
        let mut ays = vs.zs;
        let mut azs = x86_64::_mm256_sub_ps(splatf32x8(0.), vs.ys);
        let alensqs = x86_64::_mm256_add_ps(
            x86_64::_mm256_mul_ps(ays, ays),
            x86_64::_mm256_mul_ps(azs, azs),
        );
        // if a.magnitude_squared() < 0.01 {
        let alensqlts = x86_64::_mm256_cmp_ps::<{ x86_64::_CMP_LT_OQ }>(alensqs, splatf32x8(0.01));
        //     a = Vec3::new(-v.z, 0., v.x); // v × (0, 1, 0)
        x86_64::_mm256_maskstore_ps(
            &mut axs as *mut f32x8 as *mut f32,
            x86_64::_mm256_castps_si256(alensqlts),
            x86_64::_mm256_sub_ps(splatf32x8(0.), vs.zs),
        );
        x86_64::_mm256_maskstore_ps(
            &mut ays as *mut f32x8 as *mut f32,
            x86_64::_mm256_castps_si256(alensqlts),
            splatf32x8(0.),
        );
        x86_64::_mm256_maskstore_ps(
            &mut azs as *mut f32x8 as *mut f32,
            x86_64::_mm256_castps_si256(alensqlts),
            vs.xs,
        );
        // }
        // a.normalize();
        let alensqs = x86_64::_mm256_add_ps(
            x86_64::_mm256_add_ps(
                x86_64::_mm256_mul_ps(axs, axs),
                x86_64::_mm256_mul_ps(ays, ays),
            ),
            x86_64::_mm256_mul_ps(azs, azs),
        );
        let arlen = x86_64::_mm256_rsqrt_ps(alensqs);
        let axs = x86_64::_mm256_mul_ps(axs, arlen);
        let ays = x86_64::_mm256_mul_ps(ays, arlen);
        let azs = x86_64::_mm256_mul_ps(azs, arlen);

        // let b = v.cross(a);
        let bxs = x86_64::_mm256_sub_ps(
            x86_64::_mm256_mul_ps(vs.ys, azs),
            x86_64::_mm256_mul_ps(vs.zs, ays),
        );
        let bys = x86_64::_mm256_sub_ps(
            x86_64::_mm256_mul_ps(vs.zs, axs),
            x86_64::_mm256_mul_ps(vs.xs, azs),
        );
        let bzs = x86_64::_mm256_sub_ps(
            x86_64::_mm256_mul_ps(vs.xs, ays),
            x86_64::_mm256_mul_ps(vs.ys, axs),
        );

        (Vec3x8::new(axs, ays, azs), Vec3x8::new(bxs, bys, bzs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orthogonal() {
        for x in VECTORS {
            let (a, b) = orthogonal(x);
            assert!(x.dot(a).abs() < GOOD_ENOUGH, "x={} x * a = {}", x, x.dot(a));
            assert!(x.dot(b).abs() < GOOD_ENOUGH, "x={} x * b = {}", x, x.dot(b));
            assert!(x.dot(b).abs() < GOOD_ENOUGH, "x={} a * b = {}", x, a.dot(b));
        }
    }

    #[test]
    fn test_orthogonal_smarter() {
        for x in VECTORS {
            let (a, b) = orthogonal_smarter(x);
            assert!(x.dot(a).abs() < GOOD_ENOUGH, "x={} x * a = {}", x, x.dot(a));
            assert!(x.dot(b).abs() < GOOD_ENOUGH, "x={} x * b = {}", x, x.dot(b));
            assert!(x.dot(b).abs() < GOOD_ENOUGH, "x={} a * b = {}", x, a.dot(b));
        }
    }

    #[test]
    fn test_orthogonal_simd() {
        for x in VECTORS {
            let (a, b) = unsafe {
                let (a, b) = orthogonal_simd(x86_64::_mm_setr_ps(0., x.z, x.y, x.x));
                let ax = x86_64::_mm_cvtss_f32(x86_64::_mm_shuffle_ps::<3>(a, a));
                let ay = x86_64::_mm_cvtss_f32(x86_64::_mm_shuffle_ps::<2>(a, a));
                let az = x86_64::_mm_cvtss_f32(x86_64::_mm_shuffle_ps::<1>(a, a));

                let bx = x86_64::_mm_cvtss_f32(x86_64::_mm_shuffle_ps::<3>(b, b));
                let by = x86_64::_mm_cvtss_f32(x86_64::_mm_shuffle_ps::<2>(b, b));
                let bz = x86_64::_mm_cvtss_f32(x86_64::_mm_shuffle_ps::<1>(b, b));

                (Vec3::new(ax, ay, az), Vec3::new(bx, by, bz))
            };
            let errorstr = format!(
                "x={}, a={}, b={}, x*a={}, x*b={}, a*b={}",
                x,
                a,
                b,
                x.dot(a),
                x.dot(b),
                a.dot(b)
            );
            assert!(x.dot(a).abs() < GOOD_ENOUGH, "{}", errorstr);
            assert!(x.dot(b).abs() < GOOD_ENOUGH, "{}", errorstr);
            assert!(x.dot(b).abs() < GOOD_ENOUGH, "{}", errorstr);
            assert!(a.magnitude() > 0.01, "{}", errorstr);
            assert!(b.magnitude() > 0.01, "{}", errorstr);
        }
    }

    #[test]
    fn test_orthogonal_simd8() {
        unsafe {
            let mut i = 0;
            while i + 7 < VECTORS.len() {
                #[rustfmt::skip]
                let vs = Vec3x8::new(
                    x86_64::_mm256_setr_ps(
                        VECTORS[i + 0].x, VECTORS[i + 1].x, VECTORS[i + 2].x, VECTORS[i + 3].x,
                        VECTORS[i + 4].x, VECTORS[i + 5].x, VECTORS[i + 6].x, VECTORS[i + 7].x,
                    ),
                    x86_64::_mm256_setr_ps(
                        VECTORS[i + 0].y, VECTORS[i + 1].y, VECTORS[i + 2].y, VECTORS[i + 3].y,
                        VECTORS[i + 4].y, VECTORS[i + 5].y, VECTORS[i + 6].y, VECTORS[i + 7].y,
                    ),
                    x86_64::_mm256_setr_ps(
                        VECTORS[i + 0].z, VECTORS[i + 1].z, VECTORS[i + 2].z, VECTORS[i + 3].z,
                        VECTORS[i + 4].z, VECTORS[i + 5].z, VECTORS[i + 6].z, VECTORS[i + 7].z,
                    ),
                );
                let (a_s, bs) = orthogonal_simd_8(vs);

                let dot = x86_64::_mm256_add_ps(
                    x86_64::_mm256_add_ps(
                        x86_64::_mm256_mul_ps(a_s.xs, bs.xs),
                        x86_64::_mm256_mul_ps(a_s.ys, bs.ys),
                    ),
                    x86_64::_mm256_mul_ps(a_s.zs, bs.zs),
                );

                let dotltsmall =
                    x86_64::_mm256_cmp_ps::<{ x86_64::_CMP_LT_OQ }>(dot, splatf32x8(GOOD_ENOUGH));
                let dotgtnegsmall =
                    x86_64::_mm256_cmp_ps::<{ x86_64::_CMP_GT_OQ }>(dot, splatf32x8(-GOOD_ENOUGH));
                let absdotltsmall = x86_64::_mm256_and_ps(dotltsmall, dotgtnegsmall);

                #[rustfmt::skip]
                let oks: [u32; 8] = [
                    std::mem::transmute(x86_64::_mm256_cvtss_f32(x86_64::_mm256_permute_ps::<0>(absdotltsmall))),
                    std::mem::transmute(x86_64::_mm256_cvtss_f32(x86_64::_mm256_permute_ps::<1>(absdotltsmall))),
                    std::mem::transmute(x86_64::_mm256_cvtss_f32(x86_64::_mm256_permute_ps::<2>(absdotltsmall))),
                    std::mem::transmute(x86_64::_mm256_cvtss_f32(x86_64::_mm256_permute_ps::<3>(absdotltsmall))),
                    std::mem::transmute(x86_64::_mm256_cvtss_f32(x86_64::_mm256_permute_ps::<4>(absdotltsmall))),
                    std::mem::transmute(x86_64::_mm256_cvtss_f32(x86_64::_mm256_permute_ps::<5>(absdotltsmall))),
                    std::mem::transmute(x86_64::_mm256_cvtss_f32(x86_64::_mm256_permute_ps::<6>(absdotltsmall))),
                    std::mem::transmute(x86_64::_mm256_cvtss_f32(x86_64::_mm256_permute_ps::<7>(absdotltsmall))),
                ];
                for (j, ok) in oks.into_iter().enumerate() {
                    assert!(ok == 0xffffffff, "index: {}", i + j,);
                }

                i += 8;
            }
        }
    }

    extern crate test;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_orthogonal(b: &mut Bencher) {
        let mut xs = VECTORS.into_iter().cycle();
        b.iter(|| {
            black_box(orthogonal(unsafe { xs.next().unwrap_unchecked() }));
        });
    }

    #[bench]
    fn bench_orthogonal_smarter(b: &mut Bencher) {
        let mut xs = VECTORS.into_iter().cycle();
        b.iter(|| {
            black_box(orthogonal_smarter(unsafe { xs.next().unwrap_unchecked() }));
        });
    }

    #[bench]
    fn bench_orthogonal_simd(b: &mut Bencher) {
        unsafe {
            let xs = VECTORS
                .into_iter()
                .map(|x| x86_64::_mm_setr_ps(0., x.z, x.y, x.x))
                .collect::<Vec<_>>();
            let mut xs = xs.into_iter().cycle();
            b.iter(|| {
                black_box(orthogonal_simd(xs.next().unwrap_unchecked()));
            });
        }
    }

    #[bench]
    fn bench_orthogonal_simd8(b: &mut Bencher) {
        unsafe {
            let mut i = 0;
            b.iter(|| {
                #[rustfmt::skip]
                let vs = Vec3x8::new(
                    x86_64::_mm256_setr_ps(
                        VECTORS[i + 0].x, VECTORS[i + 1].x, VECTORS[i + 2].x, VECTORS[i + 3].x,
                        VECTORS[i + 4].x, VECTORS[i + 5].x, VECTORS[i + 6].x, VECTORS[i + 7].x,
                    ),
                    x86_64::_mm256_setr_ps(
                        VECTORS[i + 0].y, VECTORS[i + 1].y, VECTORS[i + 2].y, VECTORS[i + 3].y,
                        VECTORS[i + 4].y, VECTORS[i + 5].y, VECTORS[i + 6].y, VECTORS[i + 7].y,
                    ),
                    x86_64::_mm256_setr_ps(
                        VECTORS[i + 0].z, VECTORS[i + 1].z, VECTORS[i + 2].z, VECTORS[i + 3].z,
                        VECTORS[i + 4].z, VECTORS[i + 5].z, VECTORS[i + 6].z, VECTORS[i + 7].z,
                    ),
                );
                black_box(orthogonal_simd_8(vs));

                i += 8;
                if i + 7 > VECTORS.len() {
                    i = 0;
                }
            });
        }
    }

    #[bench]
    fn bench_orthogonal_simd8_same(b: &mut Bencher) {
        unsafe {
            #[rustfmt::skip]
            let vs = Vec3x8::new(
                x86_64::_mm256_setr_ps(
                    VECTORS[0].x, VECTORS[1].x, VECTORS[2].x, VECTORS[3].x,
                    VECTORS[4].x, VECTORS[5].x, VECTORS[6].x, VECTORS[7].x,
                ),
                x86_64::_mm256_setr_ps(
                    VECTORS[0].y, VECTORS[1].y, VECTORS[2].y, VECTORS[3].y,
                    VECTORS[4].y, VECTORS[5].y, VECTORS[6].y, VECTORS[7].y,
                ),
                x86_64::_mm256_setr_ps(
                    VECTORS[0].z, VECTORS[1].z, VECTORS[2].z, VECTORS[3].z,
                    VECTORS[4].z, VECTORS[5].z, VECTORS[6].z, VECTORS[7].z,
                ),
            );
            b.iter(|| {
                black_box(orthogonal_simd_8(black_box(vs)));
            });
        }
    }

    #[bench]
    fn bench_rays(b: &mut Bencher) {
        let width = 64;
        let height = 64;
        let camera = camera::Camera {
            position: Vec3::zero(),
            orientation: Quaternion::rotation_3d(1.231, Vec3::new(1., -5., 2.).normalized()),
            fov: 45f32.to_radians(),
            mapping_function: camera::MappingFunction::Linear,
        };
        let rays = camera.rays(width, height);
        b.iter(|| {
            for y in 0..height {
                for x in 0..width {
                    black_box(rays.get(black_box(x), black_box(y)));
                }
            }
        });
    }

    #[bench]
    fn bench_rays_simd(b: &mut Bencher) {
        unsafe {
            let width = 64;
            let height = 64;
            let camera = camera::Camera {
                position: Vec3::zero(),
                orientation: Quaternion::rotation_3d(1.231, Vec3::new(1., -5., 2.).normalized()),
                fov: 45f32.to_radians(),
                mapping_function: camera::MappingFunction::Linear,
            };
            let rays = camera.rays(width, height);
            let inc = x86_64::_mm256_setr_epi32(0, 1, 2, 3, 4, 5, 6, 7);
            b.iter(|| {
                for y in 0..height {
                    for x in (0..width).step_by(8) {
                        black_box(rays.get_simd(
                            x86_64::_mm256_add_epi32(x86_64::_mm256_set1_epi32(x as i32), inc),
                            y,
                        ));
                    }
                }
            });
        }
    }

    const GOOD_ENOUGH: f32 = 0.00001;

    const VECTORS: [Vec3; 50] = [
        Vec3::new(-0.9014, 1.296, 0.1079),
        Vec3::new(-2.791, 4.260, 2.298),
        Vec3::new(-0.1695, -3.197, 0.04892),
        Vec3::new(3.544, 0.2510, -0.9993),
        Vec3::new(-2.381, -4.888, -4.433),
        Vec3::new(-0.4324, -2.598, 0.4222),
        Vec3::new(0.8657, 2.076, 1.356),
        Vec3::new(0.3238, 0.8586, 0.8636),
        Vec3::new(-2.252, -2.058, 0.09055),
        Vec3::new(3.419, -3.010, 1.334),
        Vec3::new(1.042, -4.206, 0.04485),
        Vec3::new(0.3241, 1.949, -4.066),
        Vec3::new(4.836, -1.959, -0.5467),
        Vec3::new(3.236, -4.616, -2.434),
        Vec3::new(-1.411, -0.6329, -3.275),
        Vec3::new(-0.9757, -2.781, -3.321),
        Vec3::new(3.299, 0.3086, -0.4925),
        Vec3::new(0.2753, 3.034, 2.576),
        Vec3::new(-2.080, 0.7660, 0.9715),
        Vec3::new(-3.195, 1.697, -3.939),
        Vec3::new(3.393, 2.034, -2.510),
        Vec3::new(-0.03910, 4.338, -3.772),
        Vec3::new(-4.181, -4.429, 0.7391),
        Vec3::new(-1.116, 4.905, 1.152),
        Vec3::new(-0.8945, -3.278, 1.849),
        Vec3::new(-2.834, -1.629, -0.2031),
        Vec3::new(4.791, -4.263, 1.836),
        Vec3::new(0.1858, 3.279, -3.338),
        Vec3::new(2.341, 0.7779, 3.369),
        Vec3::new(3.543, -3.383, -1.797),
        Vec3::new(4.726, -1.189, 0.3897),
        Vec3::new(0.2337, -1.625, -0.9327),
        Vec3::new(2.339, -3.553, -1.658),
        Vec3::new(-2.259, -4.972, 2.651),
        Vec3::new(-1.181, 1.527, 1.337),
        Vec3::new(3.904, -0.3543, 3.115),
        Vec3::new(-3.519, -2.450, 3.460),
        Vec3::new(-0.4704, -3.362, 3.466),
        Vec3::new(-0.4454, 3.188, 2.137),
        Vec3::new(1.927, 2.379, 2.783),
        Vec3::new(-3.440, -3.248, -3.575),
        Vec3::new(-0.8908, 4.472, -4.588),
        Vec3::new(3.898, 4.979, 1.137),
        Vec3::new(2.510, -2.785, 1.089),
        Vec3::new(4.106, 3.463, 3.293),
        Vec3::new(-3.918, -4.015, 0.3724),
        Vec3::new(3.994, 1.709, -1.152),
        Vec3::new(2.622, 1.831, -3.472),
        Vec3::new(2.036, 4.260, -3.232),
        Vec3::new(-1.608, -4.838, -4.584),
    ];
}
