use crate::{
    camera::MappingFunction,
    lights::LightKind,
    material::refractive_indices,
    shapes::{BoundedPlane, Plane, Shape, ShapeKind, Sphere},
    Camera, Light, Material, Quaternion, Vec3,
};

pub fn cornellbox() -> (Camera, Vec<Shape>, Vec<Light>) {
    let mut shapes = vec![];
    // ground
    shapes.push(Shape {
        material: Material::new(Vec3::new(1., 1., 1.), 0.4, 0.4, 1., 1.),
        kind: ShapeKind::Plane(Plane {
            center: Vec3::new(0., -5., 0.),
            normal: Vec3::new(0., 1., 0.),
        }),
    });
    // red
    shapes.push(Shape {
        material: Material::new(Vec3::new(1., 0., 0.), 0.4, 0.4, 1., 1.),
        kind: ShapeKind::BoundedPlane(BoundedPlane {
            center: Vec3::new(-5., 0., 0.),
            a: Vec3::new(0., 5., 0.),
            b: Vec3::new(0., 0., 5.),
        }),
    });
    // green
    shapes.push(Shape {
        material: Material::new(Vec3::new(0., 1., 0.), 0.4, 0.4, 1., 1.),
        kind: ShapeKind::BoundedPlane(BoundedPlane {
            center: Vec3::new(5., 0., 0.),
            a: Vec3::new(0., 0., 5.),
            b: Vec3::new(0., 5., 0.),
        }),
    });
    // back
    shapes.push(Shape {
        material: Material::new(Vec3::new(1., 1., 0.8), 0.4, 0.4, 1., 1.),
        kind: ShapeKind::BoundedPlane(BoundedPlane {
            center: Vec3::new(0., 0., -5.),
            a: Vec3::new(5., 0., 0.),
            b: Vec3::new(0., 5., 0.),
        }),
    });
    // roof
    shapes.push(Shape {
        material: Material::new(Vec3::new(1., 1., 1.), 0.4, 0.4, 1., 1.),
        kind: ShapeKind::BoundedPlane(BoundedPlane {
            center: Vec3::new(0., 5., 0.),
            a: Vec3::new(5., 0., 0.),
            b: Vec3::new(0., 0., 5.),
        }),
    });
    // blue ball
    shapes.push(Shape {
        material: Material::new(Vec3::new(0., 0., 1.), 0.5, 0.04, 1., 1.),
        kind: ShapeKind::Sphere(Sphere {
            center: Vec3::new(-2.3, -3., -3.),
            radius: 2.,
        }),
    });
    // glass ball
    shapes.push(Shape {
        material: Material::new(
            Vec3::new(1., 1., 0.),
            0.1,
            0.,
            0.5,
            refractive_indices::AIR + 1.9,
            // refractive_indices::GLASS,
        ),
        kind: ShapeKind::Sphere(Sphere {
            center: Vec3::new(1.5, -3., 0.),
            radius: 2.,
        }),
    });
    (
        Camera {
            position: Vec3::new(0., 0., 15.),
            orientation: Quaternion::identity(),
            fov: 45f32.to_radians(),
            mapping_function: MappingFunction::Linear,
        },
        shapes,
        vec![Light {
            intensity: 7.,
            kind: LightKind::Point(Sphere {
                center: Vec3::new(0., 4., 0.),
                radius: 0.,
            }),
        }],
    )
}

pub fn stick_in_water() -> (Camera, Vec<Shape>, Vec<Light>) {
    let mut shapes = vec![];

    // water
    shapes.push(Shape {
        material: Material::new(
            Vec3::new(0., 0.16, 0.23),
            0.2,
            0.05,
            0.3,
            refractive_indices::WATER,
        ),
        kind: ShapeKind::Plane(Plane {
            center: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 1., 0.),
        }),
    });
    // stick
    shapes.append(&mut make_box(
        Vec3::new(1., -2., -5.),
        Quaternion::rotation_3d(45f32.to_radians(), Vec3::unit_z()),
        &Material::color(Vec3::new(0.67, 0.5, 0.29)),
        Vec3::new(0.2, 5., 0.2),
    ));
    (
        Camera {
            position: Vec3::new(0., -3., 0.),
            orientation: Quaternion::rotation_3d(30f32.to_radians(), Vec3::unit_x()),
            fov: 70f32.to_radians(),
            mapping_function: MappingFunction::Linear,
        },
        shapes,
        vec![],
    )
}

pub fn make_box(
    center: Vec3,
    orientation: Quaternion,
    material: &Material,
    size: Vec3,
) -> Vec<Shape> {
    vec![
        // +x
        Shape {
            material: material.clone(),
            kind: ShapeKind::BoundedPlane(BoundedPlane {
                center: center + orientation * Vec3::unit_x() * size.x,
                a: orientation * Vec3::new(0., size.y, 0.),
                b: orientation * Vec3::new(0., 0., size.z),
            }),
        },
        // -x
        Shape {
            material: material.clone(),
            kind: ShapeKind::BoundedPlane(BoundedPlane {
                center: center - orientation * Vec3::unit_x() * size.x,
                a: orientation * Vec3::new(0., 0., size.z),
                b: orientation * Vec3::new(0., size.y, 0.),
            }),
        },
        // +y
        Shape {
            material: material.clone(),
            kind: ShapeKind::BoundedPlane(BoundedPlane {
                center: center + orientation * Vec3::unit_y() * size.y,
                a: orientation * Vec3::new(0., 0., size.z),
                b: orientation * Vec3::new(size.x, 0., 0.),
            }),
        },
        // -y
        Shape {
            material: material.clone(),
            kind: ShapeKind::BoundedPlane(BoundedPlane {
                center: center - orientation * Vec3::unit_y() * size.y,
                a: orientation * Vec3::new(size.x, 0., 0.),
                b: orientation * Vec3::new(0., 0., size.z),
            }),
        },
        // +z
        Shape {
            material: material.clone(),
            kind: ShapeKind::BoundedPlane(BoundedPlane {
                center: center + orientation * Vec3::unit_z() * size.z,
                a: orientation * Vec3::new(size.x, 0., 0.),
                b: orientation * Vec3::new(0., size.y, 0.),
            }),
        },
        // -z
        Shape {
            material: material.clone(),
            kind: ShapeKind::BoundedPlane(BoundedPlane {
                center: center - orientation * Vec3::unit_z() * size.z,
                a: orientation * Vec3::new(0., size.y, 0.),
                b: orientation * Vec3::new(size.x, 0., 0.),
            }),
        },
    ]
}
