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
pub use shapes::Shape;

pub type Vec3 = vek::vec::repr_simd::Vec3<f32>;
pub type Vec4 = vek::vec::repr_simd::Vec4<f32>;
pub type Transform = vek::transform::repr_simd::Transform<f32, f32, f32>;
pub type Ray = vek::geom::repr_simd::Ray<f32>;
pub type Quaternion = vek::quaternion::repr_simd::Quaternion<f32>;
pub type Mat3 = vek::mat::repr_simd::column_major::Mat3<f32>;
pub type Mat4 = vek::mat::repr_simd::column_major::Mat4<f32>;

// generates two arbitrary vectors that orthogonal to `v` and each other
fn orthogonal(v: Vec3) -> (Vec3, Vec3) {
    let mut a = Vec3::new(0., v.z, -v.y); // v × (1, 0, 0)
    if a.magnitude_squared() < 0.01 {
        a = Vec3::new(-v.z, 0., v.x); // v × (0, 1, 0)
    }
    a.normalize();
    let b = v.cross(a);

    (a, b)
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
