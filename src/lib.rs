#![feature(write_all_vectored, stmt_expr_attributes, float_interpolation)]

pub mod camera;
pub mod error;
pub mod image;
pub mod material;
pub mod pixel;
pub mod shapes;
pub mod world;

pub use camera::Camera;
pub use error::Error;
pub use image::Image;
pub use material::Material;
pub use pixel::Pixel;
pub use world::World;

pub type Vec3 = vek::Vec3<f32>;
pub type Vec4 = vek::Vec4<f32>;
pub type Transform = vek::Transform<f32, f32, f32>;
pub type Ray = vek::Ray<f32>;
pub type Quaternion = vek::Quaternion<f32>;
pub type Mat3 = vek::Mat3<f32>;
pub type Mat4 = vek::Mat4<f32>;
