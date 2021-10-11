use std::f32::consts::FRAC_PI_2;

use crate::{material::refractive_indices, Material, Quaternion, Ray, Vec3};

pub struct Intersection {
    pub ray: Ray,
    pub dist: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

impl Intersection {
    pub fn reflection(&self, roughness: f32) -> Ray {
        let get_random = || {
            use rand::prelude::*;
            use rand_distr::Normal;
            if roughness == 0. {
                0.
            } else {
                rand::thread_rng()
                    .sample::<f32, _>(Normal::new(0., roughness / 3.).unwrap())
                    .clamp(-1., 1.)
            }
        };

        // A vector that never points in the same or opposite direction as
        // normal, so that it, projected on normal is not 0
        let mut v = self.normal.cross(Vec3::new(1., 0., 0.));
        if v.magnitude_squared() < 0.0001 {
            v = self.normal.cross(Vec3::new(0., 1., 0.));
        }
        // Create two vectors orthogonal to normal
        let a = (v - v.dot(self.normal) * self.normal).normalized();
        let b = self.normal.cross(a);
        let a_rot = get_random() * FRAC_PI_2;
        let b_rot = get_random() * FRAC_PI_2;
        let normal =
            Quaternion::rotation_3d(a_rot, a) * Quaternion::rotation_3d(b_rot, b) * self.normal;
        assert!(
            self.normal.dot(normal) >= 0.,
            "Randomization made normal flip direction, rotated by ({}, {}), orig normal: {}, new normal: {}, axises: {}, {}, v: {}",
            a_rot, b_rot, self.normal, normal, a, b, v,
        );

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

        // NOTE: the 'critical angle' is ignored here

        if theta2.is_nan() {
            // Total internal refraction
            Ray::new(
                self.point,
                Quaternion::rotation_3d(180f32.to_radians(), -forwards) * -self.ray.direction,
            )
        } else {
            Ray::new(self.point, Quaternion::rotation_3d(theta2, side) * forwards)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Shape {
    pub material: Material,
    pub kind: ShapeKind,
}

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
        let c2p_proj_on_a_hat = center2point.dot(a_hat) * a_hat;
        let c2p_proj_on_b_hat = center2point.dot(b_hat) * b_hat;
        if 0. < dist
            && c2p_proj_on_a_hat.magnitude_squared() <= self.a.magnitude_squared()
            && c2p_proj_on_b_hat.magnitude_squared() <= self.b.magnitude_squared()
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

#[derive(Debug, PartialEq)]
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
        if dist >= 0. && !ignore_normal.map_or(false, |n| (n - normal).is_approx_zero()) {
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
