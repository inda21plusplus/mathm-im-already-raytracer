use std::f32::consts::FRAC_PI_2;

use crate::{material, Material, Quaternion, Ray, Vec3};

pub struct Intersection {
    pub ray_dir: Vec3,
    pub dist: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

impl Intersection {
    pub fn reflection(&self, roughness: f32) -> Ray {
        // Two vectors orthogonal to normal
        let a = (self.ray_dir - self.ray_dir.dot(self.normal) * self.normal).normalized();
        let b = self.normal.cross(a);
        let a_rot = (rand::random::<f32>() * 2. - 1.) * FRAC_PI_2 * roughness;
        let b_rot = (rand::random::<f32>() * 2. - 1.) * FRAC_PI_2 * roughness;
        let normal =
            Quaternion::rotation_3d(a_rot, a) * Quaternion::rotation_3d(b_rot, b) * self.normal;

        Ray::new(
            self.point,
            Quaternion::rotation_3d(180f32.to_radians(), normal) * -self.ray_dir,
        )
    }
    pub fn refraction(&self, refractive_index: f32) -> Ray {
        let side = self.ray_dir.cross(self.normal);

        let (refractive_index_change, forwards) = if self.ray_dir.dot(self.normal) > 0. {
            (
                refractive_index / material::AIR_REFRACTIVE_INDEX,
                -self.normal,
            )
        } else {
            (
                material::AIR_REFRACTIVE_INDEX / refractive_index,
                self.normal,
            )
        };

        let theta1 = self.ray_dir.angle_between(forwards);
        let theta2 = (refractive_index_change * theta1.sin()).asin();
        let r2 = Quaternion::rotation_3d(theta2, side) * forwards;

        Ray {
            origin: self.point,
            direction: r2,
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
    fn intersection(&self, ray: Ray) -> Option<Intersection>;
}

impl Intersect for Shape {
    fn intersection(&self, ray: Ray) -> Option<Intersection> {
        match &self.kind {
            ShapeKind::Plane(p) => p.intersection(ray),
            ShapeKind::BoundedPlane(p) => p.intersection(ray),
            ShapeKind::Sphere(s) => s.intersection(ray),
        }
    }
}

/// An infinite plane
pub struct Plane {
    pub center: Vec3,
    pub normal: Vec3,
}

impl Intersect for Plane {
    fn intersection(&self, ray: Ray) -> Option<Intersection> {
        let dist = (self.center - ray.origin).dot(self.normal) / ray.direction.dot(self.normal);
        if 0. < dist && dist < 1_000_000. {
            Some(Intersection {
                ray_dir: ray.direction,
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

impl Intersect for BoundedPlane {
    fn intersection(&self, ray: Ray) -> Option<Intersection> {
        let normal = self.a.cross(self.b).normalized();
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
                ray_dir: ray.direction,
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
    fn intersection(&self, ray: Ray) -> Option<Intersection> {
        let a = ray.direction.dot(ray.origin - self.center).powi(2)
            - (ray.origin - self.center).magnitude_squared()
            + self.radius.powi(2);
        let dist = -ray.direction.dot(ray.origin - self.center) - a.sqrt();
        if dist >= 0. {
            let point = ray.origin + dist * ray.direction;
            Some(Intersection {
                ray_dir: ray.direction,
                dist,
                point,
                normal: (point - self.center).normalized(),
            })
        } else {
            None
        }
    }
}
