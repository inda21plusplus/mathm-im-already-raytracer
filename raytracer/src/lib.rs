#![feature(
    write_all_vectored,
    stmt_expr_attributes,
    float_interpolation,
    option_result_contains
)]

pub mod camera;
pub mod error;
pub mod image;
pub mod lights;
pub mod material;
pub mod pixel;
pub mod presets;
pub mod render;
pub mod shapes;

pub use camera::Camera;
pub use error::Error;
pub use image::Image;
pub use lights::Light;
pub use material::Material;
pub use pixel::Pixel;
pub use render::render;

pub type Vec3 = vek::vec::repr_simd::Vec3<f32>;
pub type Vec4 = vek::vec::repr_simd::Vec4<f32>;
pub type Transform = vek::transform::repr_simd::Transform<f32, f32, f32>;
pub type Ray = vek::geom::repr_simd::Ray<f32>;
pub type Quaternion = vek::quaternion::repr_simd::Quaternion<f32>;
pub type Mat3 = vek::mat::repr_simd::column_major::Mat3<f32>;
pub type Mat4 = vek::mat::repr_simd::column_major::Mat4<f32>;
