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
        material: {
            Material {
                color: Vec3::new(1., 1., 1.),
                specularity: 0.4,
                roughness: 0.2,
                opacity: 1.,
                refractive_index: 1.,
            }
        },
        kind: ShapeKind::Plane(Plane {
            center: Vec3::new(0., -5., 0.),
            normal: Vec3::new(0., 1., 0.),
        }),
    });
    // red
    shapes.push(Shape {
        material: {
            Material {
                color: Vec3::new(1., 0., 0.),
                specularity: 0.4,
                roughness: 0.2,
                opacity: 1.,
                refractive_index: 1.,
            }
        },
        kind: ShapeKind::BoundedPlane(BoundedPlane {
            center: Vec3::new(-5., 0., 0.),
            a: Vec3::new(0., 5., 0.),
            b: Vec3::new(0., 0., 5.),
        }),
    });
    // green
    shapes.push(Shape {
        material: {
            Material {
                color: Vec3::new(0., 1., 0.),
                specularity: 0.4,
                roughness: 0.2,
                opacity: 1.,
                refractive_index: 1.,
            }
        },
        kind: ShapeKind::BoundedPlane(BoundedPlane {
            center: Vec3::new(5., 0., 0.),
            a: Vec3::new(0., 0., 5.),
            b: Vec3::new(0., 5., 0.),
        }),
    });
    // back
    shapes.push(Shape {
        material: {
            Material {
                color: Vec3::new(1., 1., 0.8),
                specularity: 0.4,
                roughness: 0.2,
                opacity: 1.,
                refractive_index: 1.,
            }
        },
        kind: ShapeKind::BoundedPlane(BoundedPlane {
            center: Vec3::new(0., 0., -5.),
            a: Vec3::new(5., 0., 0.),
            b: Vec3::new(0., 5., 0.),
        }),
    });
    // roof
    shapes.push(Shape {
        material: {
            Material {
                color: Vec3::new(1., 1., 1.),
                specularity: 0.3,
                roughness: 0.5,
                opacity: 1.,
                refractive_index: 1.,
            }
        },
        kind: ShapeKind::BoundedPlane(BoundedPlane {
            center: Vec3::new(0., 5., 0.),
            a: Vec3::new(5., 0., 0.),
            b: Vec3::new(0., 0., 5.),
        }),
    });
    // blue ball
    shapes.push(Shape {
        material: {
            Material {
                color: Vec3::new(0., 0., 1.),
                specularity: 0.5,
                roughness: 0.04,
                opacity: 1.,
                refractive_index: 1.,
            }
        },
        kind: ShapeKind::Sphere(Sphere {
            center: Vec3::new(-2.3, -3., -3.),
            radius: 2.,
        }),
    });
    // glass ball
    shapes.push(Shape {
        material: {
            Material {
                color: Vec3::new(1., 1., 0.),
                specularity: 0.1,
                roughness: 0.,
                opacity: 0.5,
                refractive_index: refractive_indices::AIR + 1.9,
            }
        },
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
        vec![
            Light {
                intensity: 7.,
                kind: LightKind::Point(Sphere {
                    center: Vec3::new(0., 4.5, 0.),
                    radius: 0.5,
                }),
            },
            Light {
                intensity: 0.05,
                kind: LightKind::Ambient,
            },
        ],
    )
}

pub fn stick_in_water() -> (Camera, Vec<Shape>, Vec<Light>) {
    let mut shapes = vec![];

    // water
    shapes.push(Shape {
        material: {
            Material {
                color: Vec3::new(0., 0.16, 0.23),
                specularity: 0.2,
                roughness: 0.05,
                opacity: 0.3,
                refractive_index: refractive_indices::WATER,
            }
        },
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

pub fn light_and_box() -> (Camera, Vec<Shape>, Vec<Light>) {
    let mut shapes = make_box(
        Vec3::new(0., 0., -5.),
        Quaternion::identity(),
        &Material {
            color: Vec3::new(0.3, 0.5, 0.4),
            specularity: 0.5,
            roughness: 0.8,
            opacity: 1.,
            refractive_index: 1.,
        },
        Vec3::one(),
    );

    shapes.push(Shape {
        material: Material {
            color: Vec3::new(1., 1., 1.),
            specularity: 0.5,
            roughness: 0.8,
            opacity: 1.,
            refractive_index: 1.,
        },
        kind: ShapeKind::BoundedPlane(BoundedPlane {
            center: Vec3::new(0., -3., -5.),
            a: Vec3::unit_x() * 4.,
            b: -Vec3::unit_z() * 4.,
        }),
    });

    (
        Camera {
            position: Vec3::new(0., 0., 0.),
            orientation: Quaternion::identity(),
            fov: 70f32.to_radians(),
            mapping_function: MappingFunction::Linear,
        },
        shapes,
        vec![Light {
            intensity: 3.,
            kind: LightKind::Point(Sphere {
                center: Vec3::new(0., 5., -5.),
                radius: 1.,
            }),
        }],
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
