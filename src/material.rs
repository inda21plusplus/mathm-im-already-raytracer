use crate::Vec3;

pub struct Material {
    pub color: Vec3,
    pub specularity: f32,
}

impl Material {
    pub fn new(color: Vec3, reflectiveness: f32) -> Self {
        Self {
            color,
            specularity: reflectiveness,
        }
    }
    pub fn color(color: Vec3) -> Self {
        Self {
            color,
            specularity: 0.,
        }
    }
}
