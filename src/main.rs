#![deny(clippy::all)]
#![forbid(unsafe_code)]

pub mod world;

use crate::world::World;
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use web_time::{Duration, Instant};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};

use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const TARGET_FPS: f64 = 60.0;
const MAX_FLAKES_PER_SPAWN: u32 = WIDTH / 20;

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Trace).expect("error initializing logger");

        wasm_bindgen_futures::spawn_local(run());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();

        pollster::block_on(run());
    }
}

async fn run() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Rusty Night")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let window = Rc::new(window);

    #[cfg(target_arch = "wasm32")]
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

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
        Pixels::new_async(WIDTH, HEIGHT, surface_texture)
            .await
            .expect("Pixels error")
    };
    let mut world = World::new(WIDTH, HEIGHT);
    let target_fps_duration = Duration::from_secs_f64(1.0 / TARGET_FPS);
    let mut last_frame = Instant::now();
    let mut wall_clock = Instant::now();
    let mut last_spawn = Instant::now();
    let mut is_running = false;

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if !is_running {
                world.draw_debug(pixels.frame_mut());
            } else {
                world.draw_background(pixels.frame_mut());
            }

            world.draw_village(pixels.frame_mut());
            world.draw_flakes(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        let mut is_touch = false;

        match event {
            Event::WindowEvent {
                event: winit::event::WindowEvent::Touch(touch),
                ..
            } => match touch.phase {
                winit::event::TouchPhase::Started => {
                    is_touch = true;
                    println!("Touch started");
                }
                winit::event::TouchPhase::Moved => {
                    println!("Touch moved");
                }
                winit::event::TouchPhase::Ended => {
                    println!("Touch ended");
                }
                _ => {}
            },
            _ => {}
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Space)
                || input.mouse_released(0)
                || input.mouse_released(1)
                || input.mouse_released(2)
                || input.mouse_released(3)
                || is_touch
            {
                is_running = !is_running;

                wall_clock = Instant::now();
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw

            if is_running {
                let now = Instant::now();
                let elapsed = now - last_frame;
                if elapsed >= target_fps_duration {
                    last_frame = now;
                    world.update();
                    window.request_redraw();
                }
                let wall_elapsed = now - wall_clock;

                let spawn_interval = if world.max_spawned_flakes == 0 {
                    Duration::from_millis(2000)
                } else {
                    Duration::from_millis(2000 / world.max_spawned_flakes as u64)
                };
                if now - last_spawn >= spawn_interval as Duration
                    && world.max_spawned_flakes < MAX_FLAKES_PER_SPAWN
                {
                    world.max_spawned_flakes = wall_elapsed.as_secs() as u32;
                    if wall_elapsed.as_secs() < 3 {
                        world.max_flakes_total += wall_elapsed.as_secs() as usize;
                    } else if world.max_flakes_total < 10000 {
                        world.max_flakes_total *= 2;
                    }

                    last_spawn = now;
                }
            }
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
