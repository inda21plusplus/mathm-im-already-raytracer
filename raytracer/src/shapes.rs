use std::f32::consts::FRAC_PI_2;

use crate::{
    material::{self, refractive_indices},
    Material, Quaternion, Ray, Vec3,
};

pub struct Intersection {
    pub ray: Ray,
    pub dist: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

impl Intersection {
    pub fn reflection(&self, roughness: f32) -> Ray {
        // Two vectors orthogonal to normal
        let a =
            (self.ray.direction - self.ray.direction.dot(self.normal) * self.normal).normalized();
        let b = self.normal.cross(a);
        let a_rot = (rand::random::<f32>() * 2. - 1.) * FRAC_PI_2 * roughness;
        let b_rot = (rand::random::<f32>() * 2. - 1.) * FRAC_PI_2 * roughness;
        let normal =
            Quaternion::rotation_3d(a_rot, a) * Quaternion::rotation_3d(b_rot, b) * self.normal;
        assert!(self.normal.dot(normal) > 0.);

        Ray::new(
            self.point,
            Quaternion::rotation_3d(180f32.to_radians(), normal) * -self.ray.direction,
        )
    }
    pub fn refraction(&self, refractive_index: f32) -> Ray {
        let side = self.ray.direction.cross(self.normal);

        #[rustfmt::skip]
        let (refractive_index_change, forwards) = if self.ray.direction.dot(self.normal) < 0. {
            (refractive_indices::AIR / refractive_index, -self.normal)
        } else {
            (refractive_index / refractive_indices::AIR, self.normal)
        };

        let theta1 = self.ray.direction.angle_between(forwards);
        let theta2 = (refractive_index_change * theta1.sin()).asin();

        // NOTE: the 'critical angle is ignored here'

        if theta2.is_nan() {
            // Total internal refraction
            Ray::new(
                self.point,
                Quaternion::rotation_3d(180f32.to_radians(), -self.normal) * -self.ray.direction,
            )
        } else {
            Ray::new(self.point, Quaternion::rotation_3d(theta2, side) * forwards)
        }
    }
}

pub struct Shape {
    pub material: Material,
    pub kind: ShapeKind,
}

pub enum ShapeKind {
    Plane(Plane),
    BoundedPlane(BoundedPlane),
    Sphere(Sphere),
}

pub trait Intersect {
    /// Finds an intersection between the ray and `self`, if any exists in the
    /// positive direction of the ray (`dist` will be >= 0). If `ignore_normal`
    /// is `Some`, any intersection with the intersection normal ==
    /// `ignore_normal` will be ignored
    fn intersection(&self, ray: Ray, ignore_normal: Option<Vec3>) -> Option<Intersection>;
}

impl Intersect for Shape {
    fn intersection(&self, ray: Ray, ignore_normal: Option<Vec3>) -> Option<Intersection> {
        match &self.kind {
            ShapeKind::Plane(p) => p.intersection(ray, ignore_normal),
            ShapeKind::BoundedPlane(p) => p.intersection(ray, ignore_normal),
            ShapeKind::Sphere(s) => s.intersection(ray, ignore_normal),
        }
    }
}

/// An infinite plane
pub struct Plane {
    pub center: Vec3,
    pub normal: Vec3,
}

impl Intersect for Plane {
    fn intersection(&self, ray: Ray, ignore_normal: Option<Vec3>) -> Option<Intersection> {
        if ignore_normal.contains(&self.normal) {
            return None;
        }
        let dist = (self.center - ray.origin).dot(self.normal) / ray.direction.dot(self.normal);
        if 0. < dist {
            Some(Intersection {
                ray,
                dist,
                point: ray.origin + ray.direction * dist,
                normal: self.normal,
            })
        } else {
            None
        }
    }
}

pub struct BoundedPlane {
    pub center: Vec3,
    pub a: Vec3,
    pub b: Vec3,
}

impl BoundedPlane {
    fn normal(&self) -> Vec3 {
        self.a.cross(self.b).normalized()
    }
}

impl Intersect for BoundedPlane {
    fn intersection(&self, ray: Ray, ignore_normal: Option<Vec3>) -> Option<Intersection> {
        let normal = self.normal();
        if ignore_normal.contains(&normal) {
            return None;
        }
        let dist = (self.center - ray.origin).dot(normal) / ray.direction.dot(normal);
        let point = ray.origin + ray.direction * dist;
        let a_hat = self.a.normalized();
        let b_hat = self.b.normalized();
        let center2point = point - self.center;
        let c2p_proj_a_hat = center2point.dot(a_hat) * a_hat;
        let c2p_proj_b_hat = center2point.dot(b_hat) * b_hat;
        if 0. < dist
            && dist < 1_000_000.
            && c2p_proj_a_hat.magnitude_squared() <= self.a.magnitude_squared()
            && c2p_proj_b_hat.magnitude_squared() <= self.b.magnitude_squared()
        {
            Some(Intersection {
                ray,
                dist,
                point,
                normal,
            })
        } else {
            None
        }
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Intersect for Sphere {
    fn intersection(&self, ray: Ray, ignore_normal: Option<Vec3>) -> Option<Intersection> {
        let a = ray.direction.dot(ray.origin - self.center).powi(2)
            - (ray.origin - self.center).magnitude_squared()
            + self.radius.powi(2);
        let dist = -ray.direction.dot(ray.origin - self.center) - a.sqrt();
        let point = ray.origin + dist * ray.direction;
        let normal = (point - self.center).normalized();
        if dist >= 0. && !ignore_normal.contains(&normal) {
            Some(Intersection {
                ray,
                dist,
                point,
                normal,
            })
        } else {
            None
        }
    }
}
