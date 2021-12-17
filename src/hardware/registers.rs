pub struct Registers {
    r_r0: u16,
    r_r1: u16,
    r_r2: u16,
    r_r3: u16,
    r_r4: u16,
    r_r5: u16,
    r_r6: u16,
    pub r_r7: u16,
    pub r_pc: u16, /* program counter */
    r_cond: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            r_r0: 0,
            r_r1: 0,
            r_r2: 0,
            r_r3: 0,
            r_r4: 0,
            r_r5: 0,
            r_r6: 0,
            r_r7: 0,
            r_pc: 0x3000, /* program counter */
            r_cond: 0,
        }
    }
    pub fn get_val(&self, index: u16) -> u16 {
        match index {
            0 => self.r_r0,
            1 => self.r_r1,
            2 => self.r_r2,
            3 => self.r_r3,
            4 => self.r_r4,
            5 => self.r_r5,
            6 => self.r_r6,
            7 => self.r_r7,
            8 => self.r_pc,
            9 => self.r_cond,
            _ => panic!("index out of bond"),
        }
    }

    pub fn update_val(&mut self, index: u16, val: u16) -> () {
        match index {
            0 => self.r_r0 = val,
            1 => self.r_r1 = val,
            2 => self.r_r2 = val,
            3 => self.r_r3 = val,
            4 => self.r_r4 = val,
            5 => self.r_r5 = val,
            6 => self.r_r6 = val,
            7 => self.r_r7 = val,
            8 => self.r_pc = val,
            9 => self.r_cond = val,
            _ => panic!("index out of bond"),
        }
    }
}
