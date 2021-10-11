use crate::{shapes::Sphere, Ray, Vec3};

// Point light
pub struct Light {
    pub intensity: f32,
    pub kind: LightKind,
}

impl Light {
    pub fn ray_to(&self, to: Vec3) -> Option<Ray> {
        match &self.kind {
            LightKind::Point(sphere) => {
                Some(Ray::new(sphere.center, (to - sphere.center).normalized()))
            }
            LightKind::Ambient => None,
        }
    }
    pub fn lambert(&self, point: Vec3, normal: Vec3) -> f32 {
        match &self.kind {
            LightKind::Point(sphere) => {
                let point2light = sphere.center - point;
                (self.intensity * normal.dot(point2light) / point2light.magnitude_squared()).max(0.)
            }
            LightKind::Ambient => self.intensity,
        }
    }
}

pub enum LightKind {
    Point(Sphere),
    Ambient,
}
