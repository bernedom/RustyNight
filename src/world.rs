#![deny(clippy::all)]
#![forbid(unsafe_code)]

use rand::Rng;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const MAX_FLAKES_PER_SPAWN: u32 = WIDTH / 20;
const TARGET_FPS: f64 = 60.0;

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

struct SnowFlake {
    x: i16,
    y: i16,
    velocity_x: i16,
    velocity_y: i16,
}
pub struct World {
    flakes: Vec<SnowFlake>,
    rng: rand::rngs::ThreadRng,
    current_max_flakes: u32,
    tick: u32,
}

impl World {
    /// Create a new `World` instance that can draw snowflakes.
    pub fn new() -> World {
        World {
            flakes: Vec::new(),
            rng: rand::thread_rng(),
            current_max_flakes: 0,
            tick: 0,
        }
    }

    /// Update the `World` internal state; Let the flakes fall.
    pub fn update(&mut self) {
        self.tick += 1;
        for flake in self.flakes.iter_mut() {
            flake.velocity_x = self.rng.gen_range(-1..=1);

            flake.x += flake.velocity_x;
            flake.y += flake.velocity_y;
        }

        // remove all flakes that reached the bottom
        self.flakes.retain(|flake| flake.y < HEIGHT as i16);

        if self.current_max_flakes > 1 {
            let num_new_flakes = self.rng.gen_range(1..self.current_max_flakes); // spawn a random number of flakes
            println!("num_new_flakes: {}", num_new_flakes);
            for _ in 0..num_new_flakes {
                self.flakes.push(SnowFlake {
                    x: self.rng.gen_range(0..WIDTH as i16),
                    y: 1,
                    velocity_x: 0,
                    velocity_y: self.rng.gen_range(1..=2),
                });
            }
        }

        if self.current_max_flakes < MAX_FLAKES_PER_SPAWN && self.tick % TARGET_FPS as u32 == 0 {
            println!("current_max_flakes: {}", self.current_max_flakes);
            self.current_max_flakes += 1;
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw_background(&self, frame: &mut [u8]) {
        let top_color = (0x8, 0x15, 0x45, 0xff);
        let bottom_color = (0x0, 0x0, 0x0, 0xff);

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let y = (i / WIDTH as usize) as u8;

            let interpolated = lerp_rgba_u8(top_color, bottom_color, y, HEIGHT as u8);
            let rgba: [u8; 4] = interpolated.into();

            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn draw_flakes(&self, frame: &mut [u8]) {
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
