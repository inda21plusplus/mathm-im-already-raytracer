use crate::{Material, Ray, Vec3};

pub trait Shape {
    fn material(&self) -> &Material;
    fn intersection_dist(&self, ray: Ray) -> Option<f32>;
    fn intersection_point(&self, ray: Ray) -> Option<Vec3> {
        if let Some(dist) = self.intersection_dist(ray) {
            Some(ray.origin + dist * ray.direction)
        } else {
            None
        }
    }
}

/// An infinite plane
pub struct Plane {
    pub center: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Shape for Plane {
    fn material(&self) -> &Material {
        &self.material
    }
    fn intersection_dist(&self, ray: Ray) -> Option<f32> {
        let numerator = (self.center - ray.origin).dot(self.normal);
        let denominator = ray.direction.dot(self.normal);

        if denominator.abs() < 0.001 {
            None
        } else {
            let quotient = numerator / denominator;
            Some(quotient)
        }
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Shape for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }
    fn intersection_dist(&self, ray: Ray) -> Option<f32> {
        let a = ray.direction.dot(ray.direction - self.center).powi(2)
            - (ray.direction - self.center).magnitude_squared()
            + self.radius.powi(2);
        if a < 0. {
            None
        } else {
            Some(-ray.direction.dot(ray.direction - self.center) - a.sqrt())
        }
    }
}
