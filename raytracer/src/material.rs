use crate::Vec4;

pub const AIR_REFRACTIVE_INDEX: f32 = 1.000293;

pub struct Material {
    pub color: Vec4,
    pub specularity: f32,
    pub refractive_index: f32,
}

impl Material {
    pub fn new(color: Vec4, specularity: f32, refractive_index: f32) -> Self {
        Self {
            color,
            specularity,
            refractive_index,
        }
    }
    pub fn color(color: Vec4) -> Self {
        Self {
            color,
            specularity: 0.,
            refractive_index: 1.,
        }
    }
}
