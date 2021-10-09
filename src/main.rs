use std::fs::File;

use mathm_im_already_raytracer::{
    shapes::{Plane, Sphere},
    Camera, Material, Quaternion, Vec3, World,
};

fn main() {
    let world = World {
        camera: Camera {
            position: Vec3::new(0., 0., -10.),
            orientation: Quaternion::identity(),
            fov: 90f32.to_radians(),
        },
        shapes: vec![
            Box::new(Plane {
                center: Vec3::new(0., -5., 0.),
                normal: Vec3::new(0., 1., 0.),
                material: Material::color(Vec3::new(1., 1., 1.)),
            }),
            Box::new(Plane {
                center: Vec3::new(-5., 0., 0.),
                normal: Vec3::new(1., 0., 0.),
                material: Material::color(Vec3::new(1., 0., 0.)),
            }),
            Box::new(Plane {
                center: Vec3::new(5., 0., 0.),
                normal: Vec3::new(-1., 0., 0.),
                material: Material::color(Vec3::new(0., 1., 0.)),
            }),
            Box::new(Plane {
                center: Vec3::new(0., 0., 5.),
                normal: Vec3::new(0., 0., 1.),
                material: Material::color(Vec3::new(1., 1., 0.8)),
            }),
            Box::new(Plane {
                center: Vec3::new(0., 5., 0.),
                normal: Vec3::new(0., -1., 0.),
                material: Material::color(Vec3::new(1., 1., 1.)),
            }),
            Box::new(Sphere {
                center: Vec3::new(2., -4., 3. + 5.),
                radius: 1.,
                material: Material::color(Vec3::new(0., 0., 1.)),
            }),
        ],
    };
    let file = File::create("output.png").unwrap();
    world.render(1000, 1000).write(file).unwrap();
}
