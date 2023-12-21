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
        
        let num_windows_x = if width / 10 > 1 {rng.gen_range(1..=width / 10)} else {1};
        let num_windows_y = if height / 10 > 1 { rng.gen_range(1..=height / 10) } else { 1 };
        let num_windows = num_windows_x * num_windows_y;
        println!("num_windows_x: {}, num_windows_y: {}, num_windows: {}", num_windows_x, num_windows_y, num_windows);
        
        let padding = rng.gen_range(2..4);
        let window_width = 5;
        let window_height = 5;

        let lower_window_bound = rng.gen_range(1..(height / num_windows_y) - padding);
        
        let left_window_bound = padding;
        let right_window_bound = width - padding  - window_width;
        let padding_x = (right_window_bound - left_window_bound) / num_windows_x;
        
        for i in 0..num_windows {
            
            let window_y = lower_window_bound + i / num_windows_y * (padding + window_height);
            let window_x = left_window_bound + padding_x * (i % num_windows_x) + window_width * (i % num_windows_x);
            println!("window_x: {}, window_y: {}", window_x, window_y);

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
