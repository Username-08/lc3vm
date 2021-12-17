use std::io::Read;

pub struct Memory {
    mem: [u16; u16::MAX as usize],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: [0; u16::MAX as usize],
        }
    }

    pub fn read(&mut self, addr: u16) -> u16 {
        // 0xFE00 => KBSR
        // 0xFE02 => KBDR
        if addr == 0xFE00 {
            self.mem[0xFE00] = 1 << 15;
            self.mem[0xFE02] =
                std::io::stdin().bytes().next().unwrap().unwrap() as u16;
        }
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u16) -> () {
        self.mem[addr as usize] = val;
    }
}
