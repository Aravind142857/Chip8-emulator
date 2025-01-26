use rust_emu::*;

#[test]
fn it_adds_two() {
    let result = add_two(4);
    assert_eq!(result, 6);
}

#[test]
fn it_initializes_memory() {
    let memory = memory::Memory::new();
    assert_eq!(memory.memory.len(), 4096);
}
#[test]
fn it_initializes_registers() {
    let registers = registers::Registers::new();
    assert_eq!(registers.regs, [0; 16]);
    assert_eq!(registers.i, 0);
    assert_eq!(registers.delay, 0);
    assert_eq!(registers.sound, 0);
    assert_eq!(registers.pc, 0x200);
    assert_eq!(registers.sp, 0);
    assert_eq!(registers.stack, [0; 16]);
}
#[test]
fn it_initializes_display() {
    println!("Starting test");
    if let Some(display) = display::Display::new(false) {
        assert_eq!(display.display, [[0; 8]; 32]);
        assert_eq!(display.color_on, [0; 3]);
        assert_eq!(display.color_off, [0xFF; 3]);
        assert_eq!(display.keypad, [false; 16]);
    } else {
        println!("Failed to create display");
    }
}
#[test]
fn it_loads_250_in_reg_4() {
    let mut registers = registers::Registers::new();
    let mut memory = memory::Memory::new();
    let mut display = display::Display::new(false);
    memory.load_sprites();
    registers.delay = 5;
    registers.sound = 3;

    let opcode = [0x64, 0xF4];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    println!("Command executed: {:02X}{:02X}", &opcode[0], &opcode[1]);
    registers.show();
    assert_eq!(registers.get_register(4), 0xF4);
    let opcode = [0x74, 0x06];
    parse_instructions(opcode, &mut display.expect("msg"), &mut memory, &mut registers);
    println!("Command executed: {:02X}{:02X}", &opcode[0], &opcode[1]);
    registers.show();
    assert_eq!(registers.get_register(4), 0xFA);
}
#[test]
fn it_does_not_load_ten_in_reg_4() {
    let mut registers = registers::Registers::new();
    let mut memory = memory::Memory::new();
    let mut display = display::Display::new(false);
    memory.load_sprites();
    registers.delay = 5;
    registers.sound = 3;

    let opcode = [0x64, 0xF4];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    println!("Command executed: {:02X}{:02X}", &opcode[0], &opcode[1]);
    registers.show();
    assert_ne!(registers.get_register(4), 0x0A);
}
#[test]
fn it_tests_memory_and_registers() {
    let mut registers = registers::Registers::new();
    let mut memory = memory::Memory::new();
    let mut display = display::Display::new(false);
    memory.load_sprites();
    registers.delay = 5;
    registers.sound = 3;

    let opcode = [0x60, 0x09];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(0), 9, "Expected {}. Got {}", 9, registers.get_register(0));

    let opcode = [0x70, 0x10];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(0), 25, "Expected {}. Got {}", 25, registers.get_register(0));

    let opcode = [0x81, 0x00];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(1), 25, "Expected {}. Got {}", 25, registers.get_register(1));
    
    let opcode = [0x62, 0x0E];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(2), 14, "Expected {}. Got {}", 14, registers.get_register(2));

    let opcode = [0x60, 0x09];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(0), 9, "Expected {}. Got {}", 9, registers.get_register(0));

    let opcode = [0x82, 0x01];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(2), 15, "Expected {}. Got {}", 15, registers.get_register(2));

    let opcode = [0x82, 0x03];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(2), 6, "Expected {}. Got {}", 6, registers.get_register(2));

    let opcode = [0x82, 0x02];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(2), 0, "Expected {}. Got {}", 0, registers.get_register(2));

    let opcode = [0xA1, 0x11];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.i, 273, "Expected {}. Got {}", 273, registers.i);

    let opcode = [0xB0, 0x40];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.pc, 73, "Expected {}. Got {}", 73, registers.pc);

    let opcode = [0xF1, 0x1E];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.i, 298, "Expected {}. Got {}", 298, registers.i);

    let opcode = [0x63, 0xFE];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(3), 254, "Expected {}. Got {}", 254, registers.get_register(3));

    let opcode = [0x80, 0x34];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(0), 7, "Expected {}. Got {}", 7, registers.get_register(0));
    assert_eq!(registers.get_register(15), 1);

    let opcode = [0x80, 0x35];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(0), 9, "Expected {}. Got {}", 9, registers.get_register(0));
    assert_eq!(registers.get_register(15), 0);

    let opcode = [0x64, 0x0F];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(3), 254, "Expected {}. Got {}", 254, registers.get_register(3));

    let opcode = [0x84, 0x06];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(4), 7, "Expected {}. Got {}", 7, registers.get_register(4));
    assert_eq!(registers.get_register(15), 1);

    let opcode = [0x84, 0x0E];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(4), 14, "Expected {}. Got {}", 14, registers.get_register(4));
    assert_eq!(registers.get_register(15), 0);

    let opcode = [0x84, 0x17];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(4), 11, "Expected {}. Got {}", 11, registers.get_register(4));
    assert_eq!(registers.get_register(15), 1);

    let opcode = [0xF4, 0x29];
    parse_instructions(opcode, &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.i, 55);

    let opcode = [0xA0, 0xC8];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.i, 200, "Expected {}. Got {}", 200, registers.i);

    let opcode = [0xF3, 0x33];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_register(3), 254);
    assert_eq!(memory.get(3, registers.i), &[2 as u8, 5 as u8, 4 as u8] as &[u8]);

    let opcode = [0xF5, 0x55];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(memory.get(5, registers.i), &[9 as u8, 25 as u8, 0 as u8, 254 as u8, 11 as u8] as &[u8]);

    let opcode = [0x60, 0x00];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    let opcode = [0x61, 0x00];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    let opcode = [0x62, 0x00];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    let opcode = [0x63, 0x00];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    let opcode = [0x64, 0x00];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);

    let opcode = [0xF5, 0x65];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    assert_eq!(registers.get_registers(5), &[9 as u8, 25 as u8, 0 as u8, 254 as u8, 11 as u8] as &[u8]);
}

#[test]
fn it_tests_program_direction() {
    let mut registers = registers::Registers::new();
    let mut memory = memory::Memory::new();
    let mut display = display::Display::new(false);
    memory.load_sprites();
    registers.delay = 5;
    registers.sound = 3;

    let opcode = [0x60, 0x0F];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    let opcode = [0x61, 0x04];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);
    let opcode = [0x62, 0x06];
    parse_instructions(opcode,  &mut display.as_mut().expect("msg"), &mut memory, &mut registers);

    let pc = registers.pc;
    memory.store(&[0x90 as u8, 0x10, ], registers.pc);
    
}