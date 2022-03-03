// Mostly copied from: https://github.com/parasyte/pixels/blob/94a2cc2dbdba493dcbec1e99c226a06a23088319/examples/minimal-web/src/main.rs

use im_already_raytracer::{Quaternion, Vec3};
use log::error;
use pixels::{Pixels, SurfaceTexture};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

static mut OPTIONS: im_already_raytracer::render::RenderOptions =
    im_already_raytracer::render::RenderOptions {
        width: 40,
        height: 30,
        multisampling: 1,
        soft_shadow_resolution: 2,
        max_ray_depth: 3,
        use_randomness: true,
        clamp_colors: true,
    };

static mut BUTTONS: Buttons = Buttons {
    w: false,
    a: false,
    s: false,
    d: false,
    l: false,
    r: false,
    u: false,
    n: false,
};

#[derive(Debug)]
struct Buttons {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
    l: bool,
    r: bool,
    u: bool,
    n: bool,
}

#[wasm_bindgen]
pub fn set_key(key: &str, state: bool) {
    let btns = unsafe { &mut BUTTONS };
    match key {
        "w" => btns.w = state,
        "a" => btns.a = state,
        "s" => btns.s = state,
        "d" => btns.d = state,
        "ArrowLeft" => btns.l = state,
        "ArrowRight" => btns.r = state,
        "ArrowUp" => btns.u = state,
        "ArrowDown" => btns.n = state,
        _ => {}
    }
}

#[wasm_bindgen]
pub fn set_res(width: usize, height: usize) {
    let opt = unsafe { &mut OPTIONS };
    if width > 0 {
        opt.width = width;
    }
    if height > 0 {
        opt.height = height;
    }
}

#[wasm_bindgen]
pub fn set_random(r: bool) {
    let opts = unsafe { &mut OPTIONS };
    opts.use_randomness = r;
}

#[wasm_bindgen]
pub fn set_depth(d: usize) {
    let opts = unsafe { &mut OPTIONS };
    opts.max_ray_depth = d;
}

struct World {
    camera: im_already_raytracer::Camera,
    shapes: Vec<im_already_raytracer::Shape>,
    lights: Vec<im_already_raytracer::Light>,
    yaw: f32,
    pitch: f32,
}

impl World {
    fn new() -> Self {
        let (camera, shapes, lights) = im_already_raytracer::presets::cornellbox();
        Self {
            camera,
            shapes,
            lights,
            yaw: 0.,
            pitch: 0.,
        }
    }

    fn update(&mut self, dt: f32) {
        let local_forwards = self.camera.orientation * Vec3::unit_z();
        let local_right = self.camera.orientation * Vec3::unit_x();
        let movement = dt * 0.008;
        let btns = unsafe { &BUTTONS };
        if btns.w {
            self.camera.position -= local_forwards * movement;
        }
        if btns.a {
            self.camera.position -= local_right * movement;
        }
        if btns.s {
            self.camera.position += local_forwards * movement;
        }
        if btns.d {
            self.camera.position += local_right * movement;
        }

        let spinnyspin = dt * 0.001;
        if btns.l {
            self.yaw += spinnyspin;
        }
        if btns.r {
            self.yaw -= spinnyspin;
        }
        if btns.u {
            self.pitch += spinnyspin;
        }
        if btns.n {
            self.pitch -= spinnyspin;
        }
        self.camera.orientation =
            Quaternion::rotation_y(self.yaw) * Quaternion::rotation_x(self.pitch);
    }

    fn render(&self, pixels: &mut Pixels) {
        let options = unsafe { &OPTIONS };
        let pw = pixels.context().texture_extent.width as u32;
        let ph = pixels.context().texture_extent.height as u32;
        if options.width as u32 != pw || options.height as u32 != ph {
            pixels.set_clear_color(wgpu::Color::TRANSPARENT);
            pixels.resize_buffer(options.width as u32, options.height as u32);
        }
        let image =
            im_already_raytracer::render(&options, &self.camera, &self.shapes, &self.lights);
        pixels.get_frame().copy_from_slice(&image.get_raw_data());
    }
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("error initializing logger");

    wasm_bindgen_futures::spawn_local(run());
}

async fn run() {
    let event_loop = EventLoop::new();
    let window = {
        WindowBuilder::new()
            .with_title("Raytracer")
            .build(&event_loop)
            .expect("WindowBuilder error")
    };

    let window = Rc::new(window);

    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowExtWebSys;

        // Retrieve current width and height dimensions of browser client window
        let get_window_size = || {
            let client_window = web_sys::window().unwrap();
            LogicalSize::new(
                client_window.inner_width().unwrap().as_f64().unwrap(),
                client_window.inner_height().unwrap().as_f64().unwrap(),
            )
        };

        let window = Rc::clone(&window);

        // Initialize winit window with current dimensions of browser client
        window.set_inner_size(get_window_size());

        let client_window = web_sys::window().unwrap();

        // Attach winit canvas to body element
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");

        // Listen for resize event on browser client. Adjust winit window dimensions
        // on event trigger
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
            let size = get_window_size();
            window.set_inner_size(size)
        }) as Box<dyn FnMut(_)>);
        client_window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    let mut world = World::new();
    let options = unsafe { &OPTIONS };

    let mut input = WinitInputHelper::new();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
        Pixels::new_async(options.width as u32, options.height as u32, surface_texture)
            .await
            .expect("Pixels error")
    };

    let mut last = js_sys::Date::now();
    event_loop.run(move |event, _, control_flow| {
        // Render the current frame
        if let Event::RedrawRequested(_) = event {
            let now = js_sys::Date::now();
            world.update((now - last) as f32);
            world.render(&mut pixels);
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            window.request_redraw();
            last = now;
        }

        // Handle input events
        if input.update(&event) {
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
        }
    });
}
