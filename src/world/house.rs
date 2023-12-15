#![deny(clippy::all)]
#![forbid(unsafe_code)]

use rand::Rng;

struct Window {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}
pub struct House {
    x: u32,
    width: u32,
    height: u32,
    windows: Vec<Window>,
}

impl Window {
    pub fn draw(&self, frame: &mut [u8], screen_height: u32, screen_width: u32, house_x: u32) {
        let rgba: [u8; 4] = (0xff, 0xff, 0x00, 0xff).into();
        // draw box
        for x in self.x..(self.x + self.width) {
            for y in self.y..(self.y + self.height) {
                let i = (house_x + x + (screen_height - y) * screen_width) as usize * 4;
                if i + 4 < frame.len() {
                    frame[i..(i + 4)].copy_from_slice(&rgba);
                }
            }
        }
    }
}

impl House {
    pub fn new(x: u32, width: u32, height: u32) -> House {
        let mut windows = Vec::new();
        let mut rng = rand::thread_rng();
        let num_windows = rng.gen_range(1..4);
        let padding = rng.gen_range(2..4);
        let window_width = 5;
        let window_height = 5;
        let lower_floor_y = rng.gen_range(1..(height / 2) - padding);
        let upper_floor_y = rng.gen_range((height / 2)..(height - 1) - padding);

        for i in 0..num_windows {
            let window_y = if i % 2 == 0 {
                lower_floor_y
            } else {
                upper_floor_y
            };

            let window_x = padding + padding * (i / 2) + window_width * (i / 2);

            windows.push(Window {
                x: window_x,
                y: window_y,
                width: window_width,
                height: window_height,
            });
        }

        House {
            x,
            width,
            height,
            windows,
        }
    }

    pub fn draw(&self, frame: &mut [u8], screen_height: u32, screen_width: u32) {
        let rgba: [u8; 4] = (0x00, 0x00, 0x00, 0xff).into();
        // draw box
        for x in self.x..(self.x + self.width) {
            for y in 1..self.height {
                let i = (x + (screen_height - y) * screen_width) as usize * 4;
                if i + 4 < frame.len() {
                    frame[i..(i + 4)].copy_from_slice(&rgba);
                }
            }
        }

        // draw roof at an 45 degree angle
        for y in 0..self.width / 2 {
            let roof_width = self.width - y;

            for x in y..roof_width {
                let i =
                    (self.x + x + (screen_height - y - self.height) * screen_width) as usize * 4;
                if i + 4 < frame.len() {
                    frame[i..(i + 4)].copy_from_slice(&rgba);
                }
            }
        }
        for window in &self.windows {
            window.draw(frame, screen_height, screen_width, self.x);
        }
    }
}
