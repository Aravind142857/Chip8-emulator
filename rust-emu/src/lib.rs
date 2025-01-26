pub mod memory;
pub mod display;
pub mod registers;
use core::fmt;
use std::fs::{self, File};
use std::io::{self, Read};
use std::fmt::Write;

const DEBUG: bool = false;
pub fn upper_half_of(byte: &u8) -> u8 {
    return byte >> 4;
}
pub fn lower_half_of(byte: &u8) -> u8 {
    return byte & 0x0F;
}
pub fn d_bug(s: String) {
    if DEBUG {
        println!("{}", s);
    }
}
pub fn add_two(a: usize) -> usize {
    println!("Adding {}", a);
    internal_adder(a, 2)
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}
pub fn parse_instructions(opcode: [u8;2], display: &mut display::Display, memory: &mut memory::Memory, registers: &mut registers::Registers) -> Option<bool>{
    match &upper_half_of(&opcode[0]) {
        &0x00 => {
            match &opcode[1] {
                &0xE0 => {
                    d_bug(format!("{:02X}{:02X}: Clear display", opcode[0], opcode[1]));
                    display.clear();
                },
                &0xEE => {
                    d_bug(format!("{:02X}{:02X}: Return", opcode[0], opcode[1]));
                    let res = registers.ret();
                    if res {
                        return Some(false);
                    }
                },
                _ => {return Some(false)}
            }
        },
        &0x01 => {
            d_bug(format!("{:02X}{:02X}: Jump", opcode[0], opcode[1]));
            registers.jump(((((opcode[0] & 0x0F) as u16) << 8) | ((opcode[1] & 0xFF) as u16)).into())
        },
        &0x02 => {
            d_bug(format!("{:02X}{:02X}: Call", opcode[0], opcode[1]));
            registers.call(((((opcode[0] & 0x0F) as u16) << 8) | ((opcode[1] & 0xFF) as u16)).into())
        },
        &0x03 => {
            d_bug(format!("{:02X}{:02X}: Skip if equal", opcode[0], opcode[1]));
            registers.se(lower_half_of(&opcode[0]), opcode[1])
        },
        &0x04 => {
            d_bug(format!("{:02X}{:02X}: Skip if not equal", opcode[0], opcode[1]));
            registers.sne(lower_half_of(&opcode[0]), opcode[1])
        },
        &0x05 => {
            d_bug(format!("{:02X}{:02X}: Skip if equal registers", opcode[0], opcode[1]));
            registers.se_r(lower_half_of(&opcode[0]), upper_half_of(&opcode[1]))
        },
        &0x06 => {
            d_bug(format!("{:02X}{:02X}: Load instruction", opcode[0], opcode[1]));
            registers.load(lower_half_of(&opcode[0]), opcode[1])
        },
        &0x07 => {
            d_bug(format!("{:02X}{:02X}: ADD", opcode[0], opcode[1]));
            registers.add(lower_half_of(&opcode[0]), opcode[1])
        },
        &0x08 => {
            match &lower_half_of(&opcode[1]) {
                &0x00 => {
                    d_bug(format!("{:02X}{:02X}: Load register", opcode[0], opcode[1]));
                    registers.load_r(lower_half_of(&opcode[0]), upper_half_of(&opcode[1]))
                },
                &0x01 => {
                    d_bug(format!("{:02X}{:02X}: OR register", opcode[0], opcode[1]));
                    registers.or_r(lower_half_of(&opcode[0]), upper_half_of(&opcode[1]))
                },
                &0x02 => {
                    d_bug(format!("{:02X}{:02X}: AND register", opcode[0], opcode[1]));
                    registers.and_r(lower_half_of(&opcode[0]), upper_half_of(&opcode[1]))
                },
                &0x03 => {
                    d_bug(format!("{:02X}{:02X}: XOR register", opcode[0], opcode[1]));
                    registers.xor_r(lower_half_of(&opcode[0]), upper_half_of(&opcode[1]))
                },
                &0x04 => {
                    d_bug(format!("{:02X}{:02X}: ADD register", opcode[0], opcode[1]));
                    registers.add_r(lower_half_of(&opcode[0]), upper_half_of(&opcode[1]))
                },
                &0x05 => {
                    d_bug(format!("{:02X}{:02X}: SUB register", opcode[0], opcode[1]));
                    registers.sub_r(lower_half_of(&opcode[0]), upper_half_of(&opcode[1]))
                },
                &0x06 => {
                    d_bug(format!("{:02X}{:02X}: SHIFT right register", opcode[0], opcode[1]));
                    registers.shr_r(lower_half_of(&opcode[0]))
                },
                &0x07 => {
                    d_bug(format!("{:02X}{:02X}: SUB No Remainder", opcode[0], opcode[1]));
                    registers.subn_r(lower_half_of(&opcode[0]), upper_half_of(&opcode[1]))
                },
                &0x0E => {
                    d_bug(format!("{:02X}{:02X}: SHIFT Left register", opcode[0], opcode[1]));
                    registers.shl_r(lower_half_of(&opcode[0]))
                },
                _ => {return Some(false)}
            }
        },
        &0x09 => {
            d_bug(format!("{:02X}{:02X}: Skip next Instruction register", opcode[0], opcode[1]));
            registers.sne_r(lower_half_of(&opcode[0]), upper_half_of(&opcode[1]))
        },
        &0x0a => {
            d_bug(format!("{:02X}{:02X}: Registers load I", opcode[0], opcode[1]));
            registers.loadi((((lower_half_of(&opcode[0]) as u16) << 8) | (opcode[1] as u16)).into())
        },
        &0x0b => {
            d_bug(format!("{:02X}{:02X}: Jump register", opcode[0], opcode[1]));
            registers.jump_r(((((opcode[0] & 0x0F) as u16) << 8) | ((opcode[1] & 0xFF) as u16)).into())
        },
        &0x0c => {
            d_bug(format!("{:02X}{:02X}: Random byte AND", opcode[0], opcode[1]));
            registers.rand_and(lower_half_of(&opcode[0]), opcode[1])
        },
        &0x0d => {
            d_bug(format!("{:02X}{:02X}: Display sprite", opcode[0], opcode[1]));
            registers.show();
            let sprite = memory.get(lower_half_of(&opcode[1]), registers.get_i());
            let changed = display.draw(registers.get_register(lower_half_of(&opcode[0])), registers.get_register(upper_half_of(&opcode[1])), sprite);
            registers.load(15, changed);
        },
        &0x0e => match &opcode[1] {
            &0x9E => {
                d_bug(format!("{:02X}{:02X}: Skip if key pressed", opcode[0], opcode[1]));
                let k = display.key_pressed();
                if k != 0xff {
                    registers.se(lower_half_of(&opcode[0]), k);
                }
            },
            &0xA1 => {
                d_bug(format!("{:02X}{:02X}: Skip if key not pressed", opcode[0], opcode[1]));
                let k = display.key_pressed();
                if k == 0xff {
                    registers.pc += 2;
                } else {
                    registers.sne(lower_half_of(&opcode[0]), k);
                }
            },
            _ => println!("None")
        },
        &0x0f => match &opcode[1] {
            &0x07 => {
                d_bug(format!("{:02X}{:02X}: Store Delay", opcode[0], opcode[1]));
                registers.load_delay(lower_half_of(&opcode[0]))
            },
            &0x0A => {
                d_bug(format!("{:02X}{:02X}: Wait and Store key", opcode[0], opcode[1]));
                let k = display.key_pressed();
                if k == 0xff {
                    registers.repeat_instruction();
                } else {
                    registers.store_key(k, lower_half_of(&opcode[0]));
                }
            },
            &0x15 => {
                d_bug(format!("{:02X}{:02X}: Store Delay", opcode[0], opcode[1]));
                registers.store_delay(lower_half_of(&opcode[0]))
            },
            &0x18 => {
                d_bug(format!("{:02X}{:02X}: Store Sound", opcode[0], opcode[1]));
                registers.store_sound(lower_half_of(&opcode[0]))
            },
            &0x1E => {
                d_bug(format!("{:02X}{:02X}: Registers ADD to I", opcode[0], opcode[1]));
                registers.addi(lower_half_of(&opcode[0]))
            },
            &0x29 => {
                d_bug(format!("{:02X}{:02X}: Registers load sprite", opcode[0], opcode[1]));
                let val = registers.get_register(lower_half_of(&opcode[0])) as u16;
                registers.loadi(5 * val)},
            &0x33 => {
                d_bug(format!("{:02X}{:02X}: Memory store BCD", opcode[0], opcode[1]));
                let i = registers.get_i();
                let vx = registers.get_register(lower_half_of(&opcode[0]));
                memory.store_bcd(vx,i);
            },
            &0x55 => {
                d_bug(format!("{:02X}{:02X}: Memory store from registers", opcode[0], opcode[1]));
                let i = registers.get_i();
                memory.store(&registers.get_registers(lower_half_of(&opcode[0]) + 1), i);
            },
            &0x65 => {
                d_bug(format!("{:02X}{:02X}: Registers store from memory", opcode[0], opcode[1]));
                let i = registers.get_i();
                registers.store_all(&memory.get(lower_half_of(&opcode[0]) + 1, i));
            },
            _ => println!("None")
        },
        _ =>  println!("Other")
    }
    return None;
}

pub fn load_program_from_file(memory: &mut [u8; 4096], filename: &str) -> io::Result<()> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    println!("loading instructions ...");
    // let hex_values: Vec<&str> = buffer.split_whitespace().collect();
    for (i, hex) in buffer.iter().enumerate() {
        // if let Ok(byte) = u8::from_str_radix(hex, 16) {
            memory[0x200 + i] = *hex;
                // println!("{:02X}", hex);
        // } else {
            // eprintln!("Invalid hex value: {}", hex);
        // }
    }
    println!("loaded instructions ....");
    println!("------------------------");

    Ok(())
}
