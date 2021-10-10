use std::process::exit;

use im_already_raytracer::{cornellbox, Vec3, World};

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_pixels::prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 800.,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(PixelsOptions {
            width: 100,
            height: 100,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin)
        .insert_resource(cornellbox())
        .add_system(render_s.system())
        .add_system(input_s.system())
        .run();
}

fn render_s(mut pixels: ResMut<PixelsResource>, world: Res<World>) {
    let frame: &mut [u8] = pixels.pixels.get_frame();
    frame.copy_from_slice(world.render(100, 100).get_raw_data().as_ref());
}

fn input_s(
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut cursor: EventReader<MouseMotion>,
    mut world: ResMut<World>,
    mut windows: ResMut<Windows>,
    time: Res<Time>,
) {
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
    let local_forwards = world.camera.orientation * Vec3::unit_z();
    let local_right = world.camera.orientation * Vec3::unit_x();
    if locked {
        for e in cursor.iter() {
            world
                .camera
                .orientation
                .rotate_3d(e.delta.x * 0.005, Vec3::unit_y());
            world
                .camera
                .orientation
                .rotate_3d(e.delta.y * 0.005, local_right);
        }
    }
    let delta = time.delta_seconds() * 2.;
    if keyboard.pressed(KeyCode::Left) {
        world.camera.position -= local_right * delta;
    }
    if keyboard.pressed(KeyCode::Right) {
        world.camera.position += local_right * delta;
    }
    if keyboard.pressed(KeyCode::Up) {
        world.camera.position += local_forwards * delta;
    }
    if keyboard.pressed(KeyCode::Down) {
        world.camera.position -= local_forwards * delta;
    }
}
