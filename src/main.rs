#![deny(clippy::all)]
#![forbid(unsafe_code)]

pub mod world;

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};

use crate::world::World;
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

///
/// Todo
/// Add more flakes
/// limit FPS
/// Add background image and twinkling windows
/// add ground
///

fn main() -> Result<(), Error> {
    env_logger::init();
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

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
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
            world.draw_background(pixels.frame_mut());
            world.draw_village(pixels.frame_mut());
            world.draw_flakes(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Space) {
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
