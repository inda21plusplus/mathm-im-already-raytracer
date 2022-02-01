use crate::{
    orthogonal,
    shapes::{Intersect, Intersection, Sphere},
    Ray, Vec3,
};

// Point light
pub struct Light {
    pub intensity: f32,
    pub kind: LightKind,
}

impl Light {
    pub fn rays_to(&self, to: Vec3, soft_shadow_resolution: usize) -> Vec<Ray> {
        let res = soft_shadow_resolution as isize;
        let mut rays = vec![];
        match &self.kind {
            LightKind::Point(sphere) => {
                let dir_from_center = (to - sphere.center).normalized();
                for x in (-res..=res).map(|x| x as f32 / res as f32 * sphere.radius) {
                    for y in (-res..=res).map(|y| y as f32 / res as f32 * sphere.radius) {
                        let (a, b) = orthogonal(dir_from_center);
                        let origin = sphere.center + x * a + y * b;
                        rays.push(Ray::new(origin, (to - origin).normalized()));
                    }
                }
            }
            LightKind::Ambient => {}
        }
        rays
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

impl Intersect for Light {
    fn intersection(&self, ray: Ray, ignore_normal: Option<Vec3>) -> Option<Intersection> {
        match self.kind {
            LightKind::Point(sphere) => sphere.intersection(ray, ignore_normal),
            LightKind::Ambient => None,
        }
    }
}

pub enum LightKind {
    Point(Sphere),
    Ambient,
}
