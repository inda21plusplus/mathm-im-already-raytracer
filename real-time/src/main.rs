use im_already_raytracer::camera::MappingFunction;
use im_already_raytracer::render::RenderOptions;
use im_already_raytracer::shapes::Shape;
use im_already_raytracer::{presets, render, Camera, Light, Quaternion, Vec3};

use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_pixels::prelude::*;

fn main() {
    let (camera, shapes, lights) = presets::cornellbox();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin)
        .insert_resource(camera)
        .insert_resource(shapes)
        .insert_resource(lights)
        .insert_resource(RenderOptions {
            width: 128,
            height: 72,
            multisampling: 1,
            soft_shadow_resolution: 1,
            max_ray_depth: 3,
            use_randomness: false,
            clamp_colors: true,
        })
        .add_system(render_s.system())
        .add_system(input_s.system())
        .run();
}

fn render_s(
    mut pixels: ResMut<PixelsResource>,
    camera: Res<Camera>,
    shapes: Res<Vec<Shape>>,
    lights: Res<Vec<Light>>,
    render_options: Res<RenderOptions>,
) {
    let pw = pixels.pixels.context().texture_extent.width as u32;
    let ph = pixels.pixels.context().texture_extent.height as u32;
    if render_options.width as u32 != pw || render_options.height as u32 != ph {
        pixels
            .pixels
            .resize_buffer(render_options.width as u32, render_options.height as u32);
    }
    let frame: &mut [u8] = pixels.pixels.get_frame();
    frame.copy_from_slice(
        render(&render_options, &camera, &shapes, &lights)
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
    mut yaw: Local<f32>,
    mut pitch: Local<f32>,
    mut windows: ResMut<Windows>,
    time: Res<Time>,
    mut lfov: Local<f32>,
    mut render_options: ResMut<RenderOptions>,
) {
    use std::f32::consts::FRAC_PI_2;

    if camera.is_added() {
        *lfov = (camera.fov + FRAC_PI_2).tan();
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
            *yaw += e.delta.x * 0.002 * fov(*lfov);
            *pitch += e.delta.y * 0.002 * fov(*lfov);

            camera.orientation = Quaternion::rotation_y(-*yaw) * Quaternion::rotation_x(-*pitch);
        }
    }
    let move_speed = time.delta_seconds() * 4.;
    if keyboard.pressed(KeyCode::A) {
        camera.position -= local_right * move_speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.position += local_right * move_speed;
    }
    if keyboard.pressed(KeyCode::W) {
        camera.position -= local_forwards * move_speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.position += local_forwards * move_speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.position -= Vec3::unit_y() * move_speed;
    }
    if keyboard.pressed(KeyCode::E) {
        camera.position += Vec3::unit_y() * move_speed;
    }
    for e in scroll.iter() {
        *lfov -= e.y / 10.;
        camera.fov = fov(*lfov);
    }
    if keyboard.pressed(KeyCode::Left) {
        render_options.width -= (100. * time.delta_seconds()) as usize;
    }
    if keyboard.pressed(KeyCode::Right) {
        render_options.width += (100. * time.delta_seconds()) as usize;
    }
    render_options.width = render_options.width.max(0).min(2000);
    if keyboard.pressed(KeyCode::Up) {
        render_options.height -= (100. * time.delta_seconds()) as usize;
    }
    if keyboard.pressed(KeyCode::Down) {
        render_options.height += (100. * time.delta_seconds()) as usize;
    }
    render_options.height = render_options.height.max(0).min(2000);
    if keyboard.pressed(KeyCode::Key1) {
        camera.mapping_function = MappingFunction::Linear;
    }
    if keyboard.pressed(KeyCode::Key2) {
        camera.mapping_function = MappingFunction::Unlinear;
    }
    if keyboard.pressed(KeyCode::Key3) {
        camera.mapping_function = MappingFunction::Circular;
    }
    if keyboard.just_pressed(KeyCode::Minus) && render_options.multisampling > 1 {
        render_options.multisampling -= 1;
    }
    if keyboard.just_pressed(KeyCode::Equals) {
        render_options.multisampling += 1;
    }
    if keyboard.just_pressed(KeyCode::LBracket) && render_options.soft_shadow_resolution > 0 {
        render_options.soft_shadow_resolution -= 1;
    }
    if keyboard.just_pressed(KeyCode::RBracket) {
        render_options.soft_shadow_resolution += 1;
    }
    if keyboard.just_pressed(KeyCode::Semicolon) && render_options.max_ray_depth > 0 {
        render_options.max_ray_depth -= 1;
    }
    if keyboard.just_pressed(KeyCode::Apostrophe) {
        render_options.max_ray_depth += 1;
    }
    if keyboard.just_pressed(KeyCode::R) {
        render_options.use_randomness = !render_options.use_randomness;
    }
}
