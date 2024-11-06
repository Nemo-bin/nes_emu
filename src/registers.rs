pub enum Flag {
    N,
    V,
    B,
    D,
    I,
    Z,
    C,
}

pub struct Registers {
    pub a  : u8,  // General purpose
    pub x  : u8,  // General purpose
    pub y  : u8,  // General purpose
    pub pc : u16, // Program counter
    pub sp : u8,  // Stack pointer
    pub sr  : u8,  // Status Register
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a  : 0,
            x  : 0,
            y  : 0,
            pc : 0,
            sp : 0,
            sr  : 0,
        }
    }

    pub fn set_flag(&mut self, f: Flag, b: bool) {
        let mask = match f {
            Flag::N => 0b1000_0000,
            Flag::V => 0b0100_0000,
            Flag::B => 0b0001_0000,
            Flag::D => 0b0000_1000,
            Flag::I => 0b0000_0100,
            Flag::Z => 0b0000_0010,
            Flag::C => 0b0000_0001,
        };

        if b {
            self.sr |= mask;
        } else {
            self.sr &= !mask;
        }
    }
}