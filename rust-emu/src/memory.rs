pub struct Memory {
    pub memory: [u8; 4096],
    // pub system_offset: [u8; 4]
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; 4096]
            // system_offset: [0; 4],
        }
    }
    pub fn load_sprites(&mut self) {
        const SPRITE_OFFSET: usize = 0;
        let sprite_data = [0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80];
        if SPRITE_OFFSET + sprite_data.len() < 512 {
            self.memory[SPRITE_OFFSET..SPRITE_OFFSET + 80].copy_from_slice(&sprite_data);
        } else {
            println!("Error: Sprite data does not fit into reserved memory!");
        }
    }
    pub fn store_bcd(&mut self, x: u8, i: u16) {
        let digits: [u8; 3] = [(x  / 100) % 10, (x / 10) % 10, x % 10];  
        println!("units: {}, tens: {}, hundreds: {}", &digits[0], &digits[1], &digits[2]);
        self.memory[i as usize..i as usize + 3].copy_from_slice(&digits);

    }
    pub fn store(&mut self, vals: &[u8], i: u16) {
        println!("Printing memory ----");
        for j in 0..vals.len() {
            println!("{:02X}: {:02X}", i + j as u16 ,vals[j]);
        }
        println!("DONE printing memory");
        self.memory[i as usize..i as usize + vals.len()].copy_from_slice(vals);
    }
    pub fn get(&mut self, x: u8, i: u16) -> &[u8] {
        return &self.memory[i as usize..i as usize + x as usize];
    }
    pub fn fetch(&mut self, pc: u16) -> [u8; 2] {
        return self.memory[pc as usize..pc as usize +2].try_into().unwrap();
    }
}