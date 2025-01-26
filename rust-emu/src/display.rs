use minifb::{Window, WindowOptions, Key};
use std::thread;
use rodio::{OutputStream, source::{Source, SineWave}};
// use std::time::Duration;
pub struct Display {
    pub display: [[u8; 8];32],
    pub color_on: [u8; 3],
    pub color_off: [u8; 3],
    pub window: Option<Window>,
    pub keypad: [bool; 16],
    pub exit: bool
}
impl Display {
    pub fn new(create_window: bool) -> Option<Self> {
        println!("Creating Display");
        let window;
        if !create_window { window = None}
        else {window = Window::new("Test", 64, 32, WindowOptions{
            resize: false,
            // scale: minifb::Scale::X16,
            borderless: true,
            title: true,
            scale: minifb::Scale::X16,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            topmost: false,
            transparency: false,
            none: false,
        }).ok();
        }
        return Some(Display {
            display: [[0; 8];32],
            color_on: [0; 3],
            color_off: [0xFF; 3],
            window,
            keypad: [false; 16],
            exit: false,
        })
    }
    pub fn from_u8_rgb(&mut self, rgb: [u8; 3]) -> u32 {
        let (r, g, b) = (rgb[0] as u32, rgb[1] as u32, rgb[2] as u32);
        (r << 16) | (g << 8) | b
    }
    
    pub fn clear(&mut self) {
        self.display = [[0;8];32];
    }
    
    pub fn handle_key_press(&mut self) {
        if self.window.is_none() {return}
        for key in self.window.as_ref().expect("msg").get_keys() {
            match key {
                Key::Key0 =>self.keypad[0x00] = true,
                Key::Key1 =>self.keypad[0x01] = true,
                Key::Key2 =>self.keypad[0x02] = true,
                Key::Key3 =>self.keypad[0x03] = true,
                Key::Key4 =>self.keypad[0x04] = true,
                Key::Key5 =>self.keypad[0x05] = true,
                Key::Key6 =>self.keypad[0x06] = true,
                Key::Key7 =>self.keypad[0x07] = true,
                Key::Key8 =>self.keypad[0x08] = true,
                Key::Key9 =>self.keypad[0x09] = true,
                Key::A =>self.keypad[0x0A] = true,
                Key::B =>self.keypad[0x0B] = true,
                Key::C =>self.keypad[0x0C] = true,
                Key::D =>self.keypad[0x0D] = true,
                Key::E =>self.keypad[0x0E] = true,
                Key::F =>self.keypad[0x0F] = true,
                Key::Escape => self.exit = true,
                _ => ()
            }
            
        }
        for key in self.window.as_ref().expect("msg").get_keys_released() {
            match key {
                Key::Key0 =>self.keypad[0x00] = false,
                Key::Key1 =>self.keypad[0x01] = false,
                Key::Key2 =>self.keypad[0x02] = false,
                Key::Key3 =>self.keypad[0x03] = false,
                Key::Key4 =>self.keypad[0x04] = false,
                Key::Key5 =>self.keypad[0x05] = false,
                Key::Key6 =>self.keypad[0x06] = false,
                Key::Key7 =>self.keypad[0x07] = false,
                Key::Key8 =>self.keypad[0x08] = false,
                Key::Key9 =>self.keypad[0x09] = false,
                Key::A =>self.keypad[0x0A] = false,
                Key::B =>self.keypad[0x0B] = false,
                Key::C =>self.keypad[0x0C] = false,
                Key::D =>self.keypad[0x0D] = false,
                Key::E =>self.keypad[0x0E] = false,
                Key::F =>self.keypad[0x0F] = false,
                _ => ()
            }
            
        }
    }
    pub fn set_key_pressed(&mut self, key:u8) {
        self.keypad[key as usize] = true;
    }
    pub fn has_exitted(&mut self) -> bool{
        return self.exit;
    }
    pub fn key_pressed(&mut self) -> u8 {
        for i in 1..16 as u8 {
            if self.keypad[i as usize] {
                return i;
            }
        }
        return 0xff;
    }
    pub fn open(&mut self) -> bool {
        return self.window.as_ref().expect("msg").is_open();
    }
    pub fn show(&mut self) {
        let on_color = self.from_u8_rgb(self.color_on);
        let off_color = self.from_u8_rgb(self.color_off);
        let mut buffer: [u32; 64 * 32] = [off_color; 64 * 32];

        for y in 0..32 {
            for x in 0..64 {

                let byte_index = x / 8;
                let bit_index = x % 8;


                if (self.display[y][byte_index] >> (7 - bit_index)) & 1 == 1 {
                    buffer[y * 64 + x] = on_color;
                }
            }
        }

        self.window.as_mut().expect("msg").set_target_fps(60);
        self.window.as_mut().expect("msg").update_with_buffer(&buffer, 64, 32).map_err(|e| eprintln!("Error {}", e));
    }
    pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> u8{
        println!("Drawing sprite at {}, {}", x, y);
        for i in 1..sprite.len() {
            println!("{:08b}", sprite[i]);
        }
        let mut changed: u8 = 0;
        for (i, &byte) in sprite.iter().enumerate() {
            let display_y = (y as usize + i) % 32;

            for bit in 0..8 {
                let display_x = (x as usize + bit) % 64;

                let pixel_is_on = (byte >> (7 - bit)) & 1;
                let byte_index = display_x / 8;
                let bit_index = display_x % 8;
                let current_pixel = (self.display[display_y][byte_index] >> (7 - bit_index)) & 1;
                if pixel_is_on == 1 && current_pixel == 1 {
                    changed = 1;
                }
                if pixel_is_on == 1 {
                    self.display[display_y][byte_index] ^= 1 << (7 - bit_index);
                }
            }
        }
        return changed;
    }
    pub fn print_display(&mut self) {
        for i in 0..32 {
            for j in 0..8 {
                print!("{:08b}", self.display[i][j]);
            }
            println!("");
        }
    }
}