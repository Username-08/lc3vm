pub mod op;

use crate::hardware::memory::Memory;
use crate::hardware::operators::Oprtr;
use crate::hardware::registers::Registers;

pub fn run(reg: &mut Registers, mem: &Memory) -> () {
    reg.r_pc = 0x3000;
    loop {
        let instr: u16 = mem.read(reg.r_pc);
        let op = instr >> 12;
        reg.r_pc += 1;

        let cat = match op {
            0 => Oprtr::OPbr,    /* branch */
            1 => Oprtr::OPadd,   /* add  */
            2 => Oprtr::OPld,    /* load */
            3 => Oprtr::OPst,    /* store */
            4 => Oprtr::OPjsr,   /* jump register */
            5 => Oprtr::OPand,   /* bitwise and */
            6 => Oprtr::OPldr,   /* load register */
            7 => Oprtr::OPstr,   /* store register */
            8 => Oprtr::OPrti,   /* unused */
            9 => Oprtr::OPnot,   /* bitwise not */
            10 => Oprtr::OPldi,  /* load indirect */
            11 => Oprtr::OPsti,  /* store indirect */
            12 => Oprtr::OPjmp,  /* jump */
            13 => Oprtr::OPres,  /* reserved (unused) */
            14 => Oprtr::OPlea,  /* load effective address */
            15 => Oprtr::OPtrap, /* execute trap */
            _ => panic!("bad op code"),
        };
    }
}
