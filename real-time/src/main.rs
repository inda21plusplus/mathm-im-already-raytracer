use std::process::exit;

use im_already_raytracer::camera::MappingFunction;
use im_already_raytracer::shapes::Shape;
use im_already_raytracer::{presets, render, Camera, Vec3};

use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_pixels::prelude::*;

struct Scaling(pub f32);

fn main() {
    let (camera, shapes) = presets::cornellbox();
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin)
        .insert_resource(camera)
        .insert_resource(shapes)
        .insert_resource(Scaling(0.5))
        .add_system(render_s.system())
        .add_system(input_s.system())
        .run();
}

fn render_s(
    mut pixels: ResMut<PixelsResource>,
    camera: Res<Camera>,
    shapes: Res<Vec<Shape>>,
    windows: Res<Windows>,
    scaling: Res<Scaling>,
) {
    let w = windows.get(pixels.window_id).unwrap();
    let tw = (w.width() * scaling.0) as u32;
    let th = (w.height() * scaling.0) as u32;
    let pw = pixels.pixels.context().texture_extent.width as u32;
    let ph = pixels.pixels.context().texture_extent.height as u32;
    if tw != pw || th != ph {
        pixels.pixels.resize_buffer(tw, th);
    }
    let frame: &mut [u8] = pixels.pixels.get_frame();
    frame.copy_from_slice(
        render(&camera, &shapes, tw as usize, th as usize)
            .get_raw_data()
            .as_ref(),
    );
}

fn input_s(
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut cursor: EventReader<MouseMotion>,
    mut scroll: EventReader<MouseWheel>,
    mut camera: ResMut<Camera>,
    mut windows: ResMut<Windows>,
    time: Res<Time>,
    mut lfov: Local<f32>,
    mut scaling: ResMut<Scaling>,
) {
    use std::f32::consts::FRAC_PI_2;

    if camera.is_added() {
        *lfov = (camera.fov + FRAC_PI_2).tan();
    }
    if keyboard.pressed(KeyCode::Q) {
        exit(0);
    }
    let locked = windows.get_primary().unwrap().cursor_locked();
    if keyboard.just_pressed(KeyCode::Escape) || mouse.just_pressed(MouseButton::Left) {
        windows
            .get_primary_mut()
            .unwrap()
            .set_cursor_lock_mode(!locked);
        windows
            .get_primary_mut()
            .unwrap()
            .set_cursor_visibility(locked);
    }
    let local_forwards = camera.orientation * Vec3::unit_z();
    let local_right = camera.orientation * Vec3::unit_x();
    let fov = |lfov: f32| lfov.atan() + FRAC_PI_2;
    if locked {
        for e in cursor.iter() {
            camera
                .orientation
                .rotate_3d(e.delta.x * 0.002 * fov(*lfov), Vec3::unit_y());
            camera
                .orientation
                .rotate_3d(e.delta.y * 0.002 * fov(*lfov), local_right);
        }
    }
    let delta = time.delta_seconds() * 2.;
    if keyboard.pressed(KeyCode::Left) {
        camera.position -= local_right * delta;
    }
    if keyboard.pressed(KeyCode::Right) {
        camera.position += local_right * delta;
    }
    if keyboard.pressed(KeyCode::Up) {
        camera.position += local_forwards * delta;
    }
    if keyboard.pressed(KeyCode::Down) {
        camera.position -= local_forwards * delta;
    }
    for e in scroll.iter() {
        *lfov -= e.y / 10.;
        camera.fov = fov(*lfov);
    }
    if keyboard.pressed(KeyCode::M) {
        scaling.0 *= 2f32.powf(time.delta_seconds());
    }
    if keyboard.pressed(KeyCode::W) {
        scaling.0 /= 2f32.powf(time.delta_seconds());
    }
    scaling.0 = scaling.0.clamp(0.01, 1.);
    if keyboard.pressed(KeyCode::Key1) {
        camera.mapping_function = MappingFunction::Linear;
    }
    if keyboard.pressed(KeyCode::Key2) {
        camera.mapping_function = MappingFunction::Unlinear;
    }
    if keyboard.pressed(KeyCode::Key3) {
        camera.mapping_function = MappingFunction::Circular;
    }
    if keyboard.just_pressed(KeyCode::Key1) || keyboard.just_pressed(KeyCode::Key2) {
        // println!("{}", world.camera.projection.to_string());
    }
}
