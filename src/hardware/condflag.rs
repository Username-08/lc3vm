use crate::hardware::registers::Registers;

pub enum CondFlag {
    FlPOS, /* P */
    FlZRO, /* Z */
    FlNEG, /* N */
}

impl CondFlag {
    pub fn update_val(val: CondFlag) -> u16 {
        match val {
            CondFlag::FlPOS => 1 << 0,
            CondFlag::FlZRO => 1 << 1,
            CondFlag::FlNEG => 1 << 2,
        }
    }

    pub fn update_r_cond(index: u16, reg: &mut Registers) -> () {
        if reg.get_val(index) == 0 {
            reg.update_val(9, CondFlag::update_val(CondFlag::FlZRO))
        } else if reg.get_val(index) >> 15 != 0 {
            reg.update_val(9, CondFlag::update_val(CondFlag::FlNEG))
        } else {
            reg.update_val(9, CondFlag::update_val(CondFlag::FlPOS))
        }
    }
}
