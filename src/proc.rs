pub mod op;

use crate::hardware::memory::Memory;
use crate::hardware::registers::Registers;

pub fn run(instr: u16, mut reg: &mut Registers, mut mem: &mut Memory) -> () {
    let op = instr >> 12;
    match op {
        0 => op::br(instr, &mut reg),           /* branch */
        1 => op::add(instr, &mut reg),          /* add  */
        2 => op::ld(instr, &mut reg, &mut mem), /* load */
        3 => op::st(instr, &mut reg, &mut mem), /* store */
        4 => op::jsr(instr, &mut reg),          /* jump register */
        5 => op::and(instr, &mut reg),          /* bitwise and */
        6 => op::ldr(instr, &mut reg, &mut mem), /* load register */
        7 => op::str(instr, &mut reg, &mut mem), /* store register */
        8 => panic!("bad op code"),             /* unused */
        9 => op::not(instr, &mut reg),          /* bitwise not */
        10 => op::ldi(instr, &mut reg, &mut mem), /* load indirect */
        11 => op::sti(instr, &mut reg, &mut mem), /* store indirect */
        12 => op::jmp(instr, &mut reg),         /* jump */
        13 => panic!("bad op code"),            /* reserved (unused) */
        14 => op::lea(instr, &mut reg),         /* load effective address */
        15 => op::trap(instr, &mut reg, &mut mem), /* execute trap */
        _ => panic!("bad op code"),
    };
}
