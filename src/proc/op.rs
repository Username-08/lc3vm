use crate::hardware::condflag::CondFlag;
use crate::hardware::memory::Memory;
use crate::hardware::registers::Registers;
use crate::utils::get_char;
// use crate::sys::getchar::get_char;
// use crate::sys::terminal;
use std::io::{stdout, Write};

pub fn add(instr: u16, reg: &mut Registers) -> () {
    // destination register
    let dr: u16 = (instr >> 9) & 0x7;
    // first operand
    let r1: u16 = (instr >> 6) & 0x7;
    // immediate flag
    let imm_flag: u16 = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let r2 = extend_int(instr & 0x1F, 5);
        reg.update_val(dr, (reg.get_val(r1) as u32 + r2 as u32) as u16);
    } else {
        let r2 = instr & 0x7;
        reg.update_val(
            dr,
            (reg.get_val(r1) as u32 + reg.get_val(r2) as u32) as u16,
        );
    }

    CondFlag::update_r_cond(dr, reg);
}

pub fn and(instr: u16, registers: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = extend_int(instr & 0x1F, 5);
        registers.update_val(dr, registers.get_val(sr1) & imm5);
    } else {
        let sr2 = instr & 0x7;
        registers
            .update_val(dr, registers.get_val(sr1) & registers.get_val(sr2));
    }
    CondFlag::update_r_cond(dr, registers);
}

pub fn ldi(instr: u16, reg: &mut Registers, mem: &mut Memory) -> () {
    // destination register
    let dr: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = extend_int(instr & 0xFF, 9);

    // add pc_offset to r_pc and read value
    let val = mem.read(reg.get_val(8) + pc_offset);
    reg.update_val(dr, mem.read(val));

    CondFlag::update_r_cond(dr, reg);
}

pub fn str(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let offset = extend_int(instr & 0x3F, 6);
    let val: u32 = registers.get_val(sr1) as u32 + offset as u32;
    let val: u16 = val as u16;
    memory.write(val, registers.get_val(dr));
}

pub fn sti(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = extend_int(instr & 0x1ff, 9);
    let val: u32 = registers.r_pc as u32 + pc_offset as u32;
    let val: u16 = val as u16;
    let adrs = memory.read(val);
    memory.write(adrs, registers.get_val(dr));
}

pub fn not(instr: u16, registers: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    registers.update_val(dr, !registers.get_val(sr1));

    CondFlag::update_r_cond(dr, registers);
}

pub fn lea(instr: u16, registers: &mut Registers) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = extend_int(instr & 0x1ff, 9);
    let val: u32 = registers.r_pc as u32 + pc_offset as u32;
    registers.update_val(dr, val as u16);
    CondFlag::update_r_cond(dr, registers);
}

pub fn ldr(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let offset = extend_int(instr & 0x3F, 6);
    let val: u32 = registers.get_val(sr1) as u32 + offset as u32;
    registers.update_val(dr, memory.read(val as u16));
    CondFlag::update_r_cond(dr, registers);
}

pub fn ld(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = extend_int(instr & 0x1ff, 9);
    let val: u32 = pc_offset as u32 + registers.r_pc as u32;
    registers.update_val(dr, memory.read(val as u16));
    CondFlag::update_r_cond(dr, registers);
}

pub fn jsr(instr: u16, registers: &mut Registers) {
    let base_reg = (instr >> 6) & 0x7;
    let long_pc_offset = extend_int(instr & 0x7ff, 11);
    let long_flag = (instr >> 11) & 1;
    registers.r_r7 = registers.get_val(8);

    if long_flag != 0 {
        let val: u32 = registers.r_pc as u32 + long_pc_offset as u32;
        registers.r_pc = val as u16; /* JSR */
    } else {
        registers.r_pc = registers.get_val(base_reg); /* JSRR */
    }
}

pub fn jmp(instr: u16, registers: &mut Registers) {
    // also handles RET
    let base_reg = (instr >> 6) & 0x7;
    registers.r_pc = registers.get_val(base_reg);
}

pub fn br(instr: u16, registers: &mut Registers) {
    let pc_offset = extend_int((instr) & 0x1ff, 9);
    let cond_flag = (instr >> 9) & 0x7;
    if cond_flag & registers.get_val(9) != 0 {
        let val: u32 = registers.get_val(8) as u32 + pc_offset as u32;
        registers.r_pc = val as u16;
    }
}

pub fn st(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = extend_int(instr & 0x1ff, 9);
    let val: u32 = registers.r_pc as u32 + pc_offset as u32;
    let val: u16 = val as u16;
    memory.write(val, registers.get_val(dr));
}

pub fn extend_int(mut x: u16, bit_count: usize) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFF << bit_count;
    }
    x
}

/// TRAP ROUTINES
// TRAP Codes
pub fn trap(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    // terminal::turn_off_canonical_and_echo_modes();
    match instr & 0xFF {
        0x20 => {
            registers.update_val(0, get_char() as u16);
        }
        0x21 => {
            print!("{}", (registers.get_val(0) as u8) as char);
            stdout().flush().expect("Flushed.");
        }
        0x22 => {
            // /* one char per word */
            let mut index = registers.get_val(0);
            let mut c = memory.read(index);
            while c != 0x0000 {
                print!("{}", (c as u8) as char);
                index += 1;
                c = memory.read(index);
            }
            stdout().flush().expect("Flushed.");
        }
        0x23 => {
            print!("Enter a character : ");
            stdout().flush().expect("Flushed.");
            registers.update_val(0, get_char() as u16);
        }
        0x24 => {
            let mut index = registers.get_val(0);
            let mut c = memory.read(index);
            while c != 0x0000 {
                let c1 = ((c & 0xFF) as u8) as char;
                print!("{}", c1);
                let c2 = ((c >> 8) as u8) as char;
                if c2 != '\0' {
                    print!("{}", c2);
                }
                index += 1;
                c = memory.read(index);
            }
            stdout().flush().expect("Flushed.");
        }
        0x25 => {
            /* TRAP HALT */
            print!("HALT");
            stdout().flush().expect("Flushed.");
            // terminal::restore_terminal_settings();
            std::process::exit(1);
        }
        _ => {
            // terminal::restore_terminal_settings();
            std::process::exit(1);
        }
    }
    // terminal::restore_terminal_settings();
}
