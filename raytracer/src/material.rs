use crate::Vec3;

pub const AIR_REFRACTIVE_INDEX: f32 = 1.000293;

pub struct Material {
    pub color: Vec3,
    pub specularity: f32,
    pub roughness: f32,
    pub opacity: f32,
    pub refractive_index: f32,
}

impl Material {
    pub fn new(
        color: Vec3,
        specularity: f32,
        roughness: f32,
        opacity: f32,
        refractive_index: f32,
    ) -> Self {
        Self {
            color,
            specularity,
            roughness,
            opacity,
            refractive_index,
        }
    }
    pub fn color(color: Vec3) -> Self {
        Self {
            color,
            specularity: 0.,
            roughness: 0.,
            opacity: 1.,
            refractive_index: 1.,
        }
    }
}