use crate::{
    camera::MappingFunction,
    shapes::{BoundedPlane, Plane, Shape, ShapeKind, Sphere},
    Camera, Material, Quaternion, Vec3,
};

pub fn cornellbox() -> (Camera, Vec<Shape>) {
    (
        Camera {
            position: Vec3::new(0., 0., -10.),
            orientation: Quaternion::identity(),
            fov: 70f32.to_radians(),
            mapping_function: MappingFunction::Linear,
        },
        vec![
            Shape {
                material: Material::new(Vec3::new(1., 1., 1.), 0.3, 0.1, 1., 1.),
                kind: ShapeKind::Plane(Plane {
                    center: Vec3::new(0., -5., 0.),
                    normal: Vec3::new(0., 1., 0.),
                }),
            },
            Shape {
                material: Material::new(Vec3::new(1., 0., 0.), 0.4, 0.1, 1., 1.),
                kind: ShapeKind::BoundedPlane(BoundedPlane {
                    center: Vec3::new(-5., 0., 0.),
                    a: Vec3::new(0., 0., 5.),
                    b: Vec3::new(0., 5., 0.),
                }),
            },
            Shape {
                material: Material::new(Vec3::new(0., 1., 0.), 0.4, 0.1, 1., 1.),
                kind: ShapeKind::BoundedPlane(BoundedPlane {
                    center: Vec3::new(5., 0., 0.),
                    a: Vec3::new(0., 5., 0.),
                    b: Vec3::new(0., 0., 5.),
                }),
            },
            Shape {
                material: Material::new(Vec3::new(1., 1., 0.8), 0.3, 0.1, 1., 1.),
                kind: ShapeKind::BoundedPlane(BoundedPlane {
                    center: Vec3::new(0., 0., 5.),
                    a: Vec3::new(5., 0., 0.),
                    b: Vec3::new(0., 5., 0.),
                }),
            },
            Shape {
                material: Material::new(Vec3::new(1., 1., 1.), 0.3, 0.1, 1., 1.),
                kind: ShapeKind::BoundedPlane(BoundedPlane {
                    center: Vec3::new(0., 5., 0.),
                    a: Vec3::new(0., 0., 5.),
                    b: Vec3::new(5., 0., 0.),
                }),
            },
            Shape {
                material: Material::new(Vec3::new(0., 0., 1.), 0.5, 0.1, 1., 1.),
                kind: ShapeKind::Sphere(Sphere {
                    center: Vec3::new(-1.5, -3., 3.),
                    radius: 2.,
                }),
            },
            Shape {
                material: Material::new(Vec3::new(1., 1., 0.), 0., 0., 0.5, 1.458 /* glass */),
                kind: ShapeKind::Sphere(Sphere {
                    center: Vec3::new(1.5, -3., 0.),
                    radius: 2.,
                }),
            },
            Shape {
                material: Material::new(Vec3::new(1., 1., 1.), 0., 0., 0., 1.5),
                kind: ShapeKind::BoundedPlane(BoundedPlane {
                    center: Vec3::new(10., 5., 0.),
                    a: Vec3::new(5., 0., 0.),
                    b: Vec3::new(0., 5., 0.),
                }),
            },
        ],
    )
}
