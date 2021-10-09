use crate::Vec3;

pub struct Material {
    pub color: Vec3,
}

impl Material {
    pub fn color(color: Vec3) -> Self {
        Self { color }
    }
}
