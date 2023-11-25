#![deny(clippy::all)]
#![forbid(unsafe_code)]

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use rand::Rng;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

///
/// Todo
/// Add more flakes
/// limit FPS
/// Add background image and twinkling windows
/// add ground
///
struct SnowFlake {
    x: i16,
    y: i16,
    velocity_x: i16,
    velocity_y: i16,
}
struct World {
    flakes: Vec<SnowFlake>,
    rng: rand::rngs::ThreadRng,
}

fn lerp_rgba_u8(
    start: (u8, u8, u8, u8),
    end: (u8, u8, u8, u8),
    index: u8,
    height: u8,
) -> (u8, u8, u8, u8) {
    let t = f64::from(index) / f64::from(height);
    let r = ((1.0 - t) * f64::from(start.0) + t * f64::from(end.0)) as u8;
    let g = ((1.0 - t) * f64::from(start.1) + t * f64::from(end.1)) as u8;
    let b = ((1.0 - t) * f64::from(start.2) + t * f64::from(end.2)) as u8;
    let a = ((1.0 - t) * f64::from(start.3) + t * f64::from(end.3)) as u8;
    (r, g, b, a)
}

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
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw_background(pixels.frame_mut());
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
    /// Create a new `World` instance that can draw snowflakes.
    fn new() -> World {
        let mut rng = rand::thread_rng();
        let mut flakes = Vec::new();
        for position in 0..(WIDTH as i16) / 10 {
            flakes.push(SnowFlake {
                x: position * 10,
                y: 1,
                velocity_x: 0,
                velocity_y: rng.gen_range(1..=2),
            });
        }
        World { flakes, rng: rng }
    }

    /// Update the `World` internal state; Let the flakes fall.
    fn update(&mut self) {
        for flake in self.flakes.iter_mut() {
            flake.velocity_x = self.rng.gen_range(-1..=1);

            flake.x += flake.velocity_x;
            flake.y += flake.velocity_y;
        }

        // remove all flakes that reached the bottom
        self.flakes.retain(|flake| flake.y < HEIGHT as i16);

        let num_new_flakes = self.rng.gen_range(1..WIDTH as i16 / 20); // spawn a random number of flakes
        for _ in 0..num_new_flakes {
            self.flakes.push(SnowFlake {
                x: self.rng.gen_range(0..WIDTH as i16),
                y: 1,
                velocity_x: 0,
                velocity_y: self.rng.gen_range(1..=2),
            });
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw_background(&self, frame: &mut [u8]) {
        let top_color = (0x8, 0x15, 0x45, 0xff);
        let bottom_color = (0x0, 0x0, 0x0, 0xff);

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let y = (i / WIDTH as usize) as u8;

            let interpolated = lerp_rgba_u8(top_color, bottom_color, y, HEIGHT as u8);
            let rgba: [u8; 4] = interpolated.into();

            pixel.copy_from_slice(&rgba);
        }
    }

    fn draw_flakes(&self, frame: &mut [u8]) {
        for flake in self.flakes.iter() {
            if flake.x < 0 || flake.x >= WIDTH as i16 {
                continue;
            }
            let x = flake.x as usize;
            let y = flake.y as usize;
            let rgba = [0xff, 0xff, 0xff, 0xff];
            let i = (x + y * WIDTH as usize) * 4;
            if i + 4 < frame.len() {
                frame[i..(i + 4)].copy_from_slice(&rgba);
            }
        }
    }
}
