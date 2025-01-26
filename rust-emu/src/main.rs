

use std::{thread, time::{Duration, Instant}};
use rust_emu::{display, load_program_from_file, memory, parse_instructions, registers};

pub fn run() {
    let mut memory = memory::Memory::new();
    let r = load_program_from_file(&mut memory.memory, "maze.ch8").map_err(|e| eprintln!("Unable to process file {}", e));
    
    let mut registers = registers::Registers::new();    
    let mut last_time = Instant::now();
    let mut display = display::Display::new(true).expect("cannot create display");
    registers.delay = 8;
    registers.sound = 10;
    memory.load_sprites();
    while display.open() {
        let opcode = memory.fetch(registers.pc);
        // println!("{:02X}, {:02X}", opcode[0], opcode[1]);
        display.handle_key_press();
        if display.has_exitted() {
            registers.show();
            return;
        }

        registers.pc += 2;
        let res = parse_instructions(opcode, &mut display, &mut memory, &mut registers);
        if !res.is_none() {
            return;
        }
        display.show();
        let elapsed = last_time.elapsed();
        if elapsed < Duration::from_millis(16) {
            std::thread::sleep(Duration::from_millis(16) - elapsed);
        }
        registers.update_timers();
        last_time = Instant::now();
    }
}
fn main() {
    run();
    // let mut memory = memory::Memory::new();
    // let mut registers = registers::Registers::new();
    // if let Some( mut display) = display::Display::new(true) {
    //     memory.load_sprites();
    //     registers.delay = 5;
    //     registers.sound = 3;

    //     let opcode = [0x60, 0xF4];
    //     parse_instructions(opcode, &mut display, &mut memory, &mut registers);
    //     println!("Command executed: {:02X}{:02X}", &opcode[0], &opcode[1]);
    //     registers.show();
    //     assert_eq!(registers.get_register(0), 0xF4);
    //     let opcode = [0x70, 0x06];
    //     parse_instructions(opcode, &mut display, &mut memory, &mut registers);
    //     println!("Command executed: {:02X}{:02X}", &opcode[0], &opcode[1]);
    //     registers.show();
    //     assert_eq!(registers.get_register(0), 0xFA);
    // } else {
    //     eprint!("Failed to create display");
    // }


    // let mut memory = memory::Memory::new();
    // let mut registers = registers::Registers::new();
    // let mut display = display::Display::new();
    // registers.delay = 5;
    // registers.sound = 3;
    // memory.load_sprites();

    // // Tested load, add
    // // let opcodes:[[u8;2];2] = [[0x60, 0xF4], [0x70, 0x06]];
    // // Tested load 02 into v1, load addr of 02 sprite into i, store 24 at v2, store 32 at v3, draw 5 byte sprite stored at i starting at v2, v3. 
    // // let opcodes:[[u8;2];5] = [[0x61, 0x0f], [0xF1, 0x29], [0x62, 0x18], [0x63, 0x20], [0xD2, 0x35]];
    // let opcodes:[[u8;2];2] = [[0x60, 0xF4], [0x70, 0x06]];
    // for &opcode in opcodes.iter() {
    //     rust_emu::parse_instructions(opcode, &mut display, &mut memory, &mut registers);
    //     println!("Command executed: {:02X}{:02X}", &opcode[0], &opcode[1]);
    //     registers.show();
    // }





    // // display.show();
    // const REFRESH_RATE: u64 = 60;
    // let mut last_update = Instant::now();
    // // loop {
    // while display.open() {
    //     display.show();
    // }
    //     let elapsed = last_update.elapsed();
    //     let elapsed_ms = elapsed.as_secs_f64() * 1000.0;
    //     if elapsed_ms > 1000.0 / REFRESH_RATE as f64 {
    //         registers.update_timers();
    //         last_update = Instant::now();
    //     }
    //     thread::sleep(Duration::from_millis(1));
    // }
}
