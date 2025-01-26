use rand::Rng;
use rodio::{OutputStream, OutputStreamHandle, source::{Source, SineWave}};
use std::time::Duration;
use std::thread;
pub struct Registers {
    pub regs: [u8; 16],
    pub i: u16,
    pub delay: u8,
    pub sound: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub stream: RodioStream,

}
type RodioStream = (OutputStream, OutputStreamHandle);
impl Registers {
    pub fn new() -> Self {
        Registers {
            regs: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            stream: OutputStream::try_default().unwrap(),
        }
    }
    pub fn ret(&mut self) -> bool{
        if self.sp == 0 {
            return false;
        }
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        return true;
    }
    pub fn call(&mut self, addr: u16) {
        self.pc += 2;
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }
    pub fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }
    pub fn se(&mut self, x: u8, kk: u8) {
        if self.regs[x as usize] == kk {
            self.pc += 2;
        }
    }
    pub fn sne(&mut self, x: u8, kk: u8) {
        if self.regs[x as usize] != kk {
            self.pc += 2;
        }
    }
    pub fn se_r(&mut self, x: u8, y: u8) {
        if self.regs[x as usize] == self.regs[y as usize] {
            self.pc += 2;
        }
    }
    pub fn load(&mut self, x: u8, kk: u8) {
        self.regs[x as usize] = kk;
    }
    pub fn add(&mut self, x: u8, kk: u8) {
        self.regs[x as usize] += kk;
    }
    pub fn load_r(&mut self, x: u8, y: u8) {
        self.regs[x as usize] = self.regs[y as usize];
    }
    pub fn or_r(&mut self, x:u8, y:u8) {
        self.regs[x as usize] = self.regs[x as usize] | self.regs[y as usize];
    }
    pub fn and_r(&mut self, x:u8, y:u8) {
        self.regs[x as usize] = self.regs[x as usize] & self.regs[y as usize];
    }
    pub fn xor_r(&mut self, x:u8, y:u8) {
        self.regs[x as usize] = self.regs[x as usize] ^ self.regs[y as usize];
    }
    pub fn add_r(&mut self, x:u8, y:u8) {
        self.regs[x as usize] = self.regs[x as usize].wrapping_add(self.regs[y as usize]);
        if self.regs[x as usize] < self.regs[y as usize]{
            self.regs[15 as usize] = 1;
        } else {
            self.regs[15 as usize] = 0;
        }
    }
    pub fn sub_r(&mut self, x:u8, y:u8) {
        if self.regs[x as usize] > self.regs[y as usize] {
            self.regs[15 as usize] = 1;
        } else {
            self.regs[15 as usize] = 0;
        }
        self.regs[x as usize] = self.regs[x as usize].wrapping_sub(self.regs[y as usize]);
    }
    pub fn shr_r(&mut self, x:u8) {
        if self.regs[x as usize] % 2 == 1 {
            self.regs[15 as usize] = 1;
        } else {
            self.regs[15 as usize] = 0;
        }
        self.regs[x as usize] = self.regs[x as usize] >> 1;
    }
    pub fn subn_r(&mut self, x:u8, y:u8) {
        if self.regs[y as usize] > self.regs[x as usize] {
            self.regs[15 as usize] = 1;
        } else {
            self.regs[15 as usize] = 0;
        }
        self.regs[x as usize] = self.regs[y as usize] - self.regs[x as usize];
    }
    pub fn shl_r(&mut self, x:u8) {
        self.regs[15 as usize] = self.regs[x as usize] >> 7;
        self.regs[x as usize] = self.regs[x as usize] << 1;
    }
    pub fn sne_r(&mut self, x: u8, y: u8) {
        if self.regs[x as usize] != self.regs[y as usize] {
            self.pc += 2;
        }
    }
    pub fn loadi(&mut self, nnn: u16) {
        println!("Setting i to {}", nnn);
        self.i = nnn;
    }
    pub fn jump_r(&mut self, nnn: u16) {
        self.pc = nnn + self.regs[0 as usize] as u16;
    }
    pub fn rand_and(&mut self, x: u8, kk:u8) {
        let mut rng = rand::thread_rng();
        let random_byte: u8 = rng.gen();
        self.regs[x as usize] = random_byte & kk;
    }
    pub fn load_delay(&mut self, x: u8) {
        self.regs[x as usize] = self.delay;
    }
    pub fn addi(&mut self, x: u8) {
        self.i += self.regs[x as usize] as u16;
    }
    pub fn store_delay(&mut self, x: u8) {
        self.delay = self.regs[x as usize];
    }
    pub fn store_sound(&mut self, x: u8) {
        self.sound = self.regs[x as usize];
    }
    pub fn get_i(&mut self) -> u16 {
        return self.i ;
    }
    pub fn get_register(&mut self, x:u8) -> u8 {
        return self.regs[x as usize];
    }
    pub fn get_registers(&mut self, x: u8) -> &[u8] {
        return &self.regs[0..x as usize];
    }
    pub fn store_all(&mut self, vals: &[u8]) {
        self.regs[0..vals.len()].copy_from_slice(vals);
    }
    pub fn store_key(&mut self, key: u8, x: u8) {
        self.regs[x as usize] = key;
    }
    pub fn repeat_instruction(&mut self) {
        self.pc -= 2;
    }
    pub fn update_timers(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            self.sound -= 1;
            self.buzz();
        }
    }
    pub fn buzz(& self) {
        let beep_frequency = 1000.0; // 1000 Hz for the beep
        let beep_duration = 1.0; // Duration of beep in seconds
        let frame_duration = 1.0 / 60.0; // Assuming a 60 FPS refresh rate
        // thread::spawn(move || {
            // let (_stream, streamhandle) = OutputStream::try_default().unwrap();
            let beep = SineWave::new(beep_frequency).take_duration(Duration::from_secs_f32(beep_duration));
            self.stream.1.play_raw(beep.convert_samples()).unwrap();
            // println!("BUZZ!");
        // });
    }
    pub fn show(& self) {
        println!("Displaying the registers...");
        for &value in self.regs.iter() {
            print!("{} ", value);
        }
        println!();
        println!("i: {}", self.i);

        println!("---------------------------");
    }
}
