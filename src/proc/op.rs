use crate::hardware::condflag::CondFlag;
use crate::hardware::memory::Memory;
use crate::hardware::registers::Registers;

pub fn add(instr: &u16, reg: &mut Registers) -> () {
    // destination register
    let dr: u16 = (instr >> 9) & 0x7;
    // first operand
    let r1: u16 = (instr >> 6) & 0x7;
    // immediate flag
    let imm_flag: u16 = (instr >> 5) & 0x1;

    if imm_flag != 0 {
        let r2 = extend_int(instr & 0x1F, 5);
        reg.update_val(dr, reg.get_val(r1) + r2);
    } else {
        let r2 = instr & 0x7;
        reg.update_val(dr, reg.get_val(r1) + r2);
    }

    CondFlag::update_r_cond(dr, reg);
}

pub fn ldi(instr: &u16, reg: &mut Registers, mem: &Memory) -> () {
    // destination register
    let dr: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = extend_int(instr & 0xFF, 9);

    // add pc_offset to r_pc and read value
    reg.update_val(dr, mem.read(mem.read(reg.get_val(8) + pc_offset)));

    CondFlag::update_r_cond(dr, reg);
}

pub fn extend_int(mut x: u16, bit_count: usize) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFF << bit_count;
    }
    x
}
