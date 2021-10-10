use std::{fs::File, time::Instant};

use mathm_im_already_raytracer::{
    shapes::{BoundedPlane, Plane, Shape, ShapeKind, Sphere},
    Camera, Error, Material, Quaternion, Vec3, Vec4, World,
};

fn main() -> Result<(), Error> {
    let world = World {
        camera: Camera {
            position: Vec3::new(0., 0., -10.),
            orientation: Quaternion::identity(),
            fov: 90f32.to_radians(),
        },
        shapes: vec![
            Shape {
                material: Material::color(Vec4::new(1., 1., 1., 1.)),
                kind: ShapeKind::BoundedPlane(BoundedPlane {
                    center: Vec3::new(0., -5., 0.),
                    a: Vec3::new(5., 0., 0.),
                    b: Vec3::new(0., 0., 5.),
                }),
            },
            Shape {
                material: Material::color(Vec4::new(1., 0., 0., 1.)),
                kind: ShapeKind::BoundedPlane(BoundedPlane {
                    center: Vec3::new(-5., 0., 0.),
                    a: Vec3::new(0., 0., 5.),
                    b: Vec3::new(0., 5., 0.),
                }),
            },
            Shape {
                material: Material::color(Vec4::new(0., 1., 0., 1.)),
                kind: ShapeKind::BoundedPlane(BoundedPlane {
                    center: Vec3::new(5., 0., 0.),
                    a: Vec3::new(0., 5., 0.),
                    b: Vec3::new(0., 0., 5.),
                }),
            },
            Shape {
                material: Material::color(Vec4::new(1., 1., 0.8, 1.)),
                kind: ShapeKind::Plane(Plane {
                    center: Vec3::new(0., 0., 5.),
                    normal: Vec3::new(0., 0., 1.),
                }),
            },
            Shape {
                material: Material::color(Vec4::new(1., 1., 1., 1.)),
                kind: ShapeKind::BoundedPlane(BoundedPlane {
                    center: Vec3::new(0., 5., 0.),
                    a: Vec3::new(0., 0., 5.),
                    b: Vec3::new(5., 0., 0.),
                }),
            },
            Shape {
                material: Material::new(Vec4::new(0., 0., 1., 1.), 0.5, 1.),
                kind: ShapeKind::Sphere(Sphere {
                    center: Vec3::new(-1.5, -3., 3.),
                    radius: 2.,
                }),
            },
            Shape {
                material: Material::new(Vec4::new(1., 0., 1., 1.), 0.5, 1.),
                kind: ShapeKind::Sphere(Sphere {
                    center: Vec3::new(1.5, -3., 0.),
                    radius: 2.,
                }),
            },
        ],
    };

    let start = Instant::now();
    let image = world.render(1000, 1000);
    let dur = start.elapsed();
    println!("{}ms", dur.as_millis());
    let data = image.get_raw_data();
    let file = File::create("output.png").unwrap();
    let mut encoder = png::Encoder::new(file, image.width as u32, image.heigth as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.write_header()?.write_image_data(&data)?;

    Ok(())
}
