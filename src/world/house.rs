#![deny(clippy::all)]
#![forbid(unsafe_code)]

pub struct House {
    x: u32,
    width: u32,
    height: u32,
}

impl House {
    pub fn new(x: u32, width: u32, height: u32) -> House {
        House { x, width, height }
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
    }
}
