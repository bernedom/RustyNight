#![deny(clippy::all)]
#![forbid(unsafe_code)]

use rand::Rng;

struct Window {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    is_lit: bool,
}
pub struct House {
    x: u32,
    width: u32,
    height: u32,
    windows: Vec<Window>,
}

impl Window {
    pub fn draw(&self, frame: &mut [u8], screen_height: u32, screen_width: u32, house_x: u32) {
        if !self.is_lit {
            return;
        }
        
        let rgba: [u8; 4] = (0xf5, 0xce, 0x42, 0xff).into();
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

        let padding = rng.gen_range(3..5);
        let window_width = 5;
        let window_height = 5;

        
        let num_windows_x = width / (window_width + padding);
        let num_windows_y = height / (window_height + padding);
                
        let lower_window_bound = height / num_windows_y / 2 - padding / 2;
        let left_window_bound = width / num_windows_x / 2 - padding / 2;
        

        for x in 0 ..num_windows_x{
            let window_x = left_window_bound + padding * x + window_width * x;
            for y in 0..num_windows_y {
                let window_y = lower_window_bound + y * (padding + window_height);
                windows.push(Window {
                    x: window_x,
                    y: window_y,
                    width: window_width,
                    height: window_height,
                    is_lit: rng.gen(),
                });
            }
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
