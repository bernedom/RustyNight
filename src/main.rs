#![deny(clippy::all)]
#![forbid(unsafe_code)]

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

/// Representation of the application state. In this example, a box will bounce around the screen.
///

struct SnowFlake {
    x: i16,
    y: i16,
    velocity_x: i16,
    velocity_y: i16,
}
struct World {
    flakes: Vec<SnowFlake>,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
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
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut());
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

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> World {
        let mut flakes = Vec::new();
        for position in 0..(WIDTH as i16) / 10 {
            flakes.push(SnowFlake {
                x: position * 10,
                y: 1,
                velocity_x: 0,
                velocity_y: 1,
            });
        }
        World { flakes }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        for flake in self.flakes.iter_mut() {
            if flake.x <= 0 || flake.x >= WIDTH as i16 {
                flake.velocity_x *= -1;
            }
            if flake.y >= HEIGHT as i16 {
                flake.y = 0;
            }

            flake.x += flake.velocity_x;
            flake.y += flake.velocity_y;
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let mut is_flake = false;
            for flake in self.flakes.iter() {
                if x == flake.x && y == flake.y {
                    is_flake = true;
                    break;
                }
            }
            let rgba = if is_flake {
                [0xff, 0xff, 0xff, 0xff]
            } else {
                [0x8, 0x15, 0x45, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
