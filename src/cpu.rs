use crate::registers::Registers;
use crate::memory::Memory;

#[derive(PartialEq)]
pub enum AddrMode {
    A,
    Abs,
    AbsX,
    AbsY,
    Imm,
    Ind,
    XInd,
    IndY,
    Rel,
    Zpg,
    ZpgX,
    ZpgY,
}

pub struct CPU {
    registers : Registers,
    memory    : Memory,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers : Registers::new(),
            memory    : Memory::new(),
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let address       = self.registers.pc;
        let data          = self.memory.read(address);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        return data;
    }

    pub fn fetchWord(&mut self) -> u16 {
        let lo_byte = self.fetch() as u16;
        let hi_byte = self.fetch() as u16;
        let data    = (hi_byte << 8) | lo_byte;
        return data;
    }

    // Functions to generalise addressing modes.
    fn read_data(&mut self, mode: AddrMode) -> u8 {
        match mode {
            AddrMode::A   => {
                self.registers.a
            },
            AddrMode::Imm => {
                self.fetch()
            },
            _             => {
                let address = self.get_address(mode);
                self.memory.read(address)
            },
        }
    }

    fn write_data(&mut self, mode: AddrMode, data: u8) {
        match mode {
            AddrMode::A => {
                self.registers.a = data;
            },
            _           => {
                let address = self.get_address(mode);
                self.memory.write(address, data);
            }
        }
    }

    fn get_address(&mut self, mode: AddrMode) -> u16 {
        match mode {
            AddrMode::Abs  => {
                self.fetchWord()
            },
            AddrMode::AbsX => {
                self.fetchWord().wrapping_add(self.registers.x as u16)
            },
            AddrMode::AbsY => {
                self.fetchWord().wrapping_add(self.registers.y as u16)
            },
            AddrMode::Ind  => {
                let address = self.fetchWord();
                self.memory.readWord(address)
            },
            AddrMode::XInd => {
                let address = self.fetchWord().wrapping_add(self.registers.x as u16);
                self.memory.readWord(address)
            },
            AddrMode::IndY => {
                let address = self.fetchWord();
                self.memory.readWord(address).wrapping_add(self.registers.y as u16)
            },
            AddrMode::Rel  => {
                let offset = self.fetch();
                let signed_offset = offset as i8;
                self.registers.pc.wrapping_add(signed_offset as u16)
            },
            AddrMode::Zpg  => {
                self.fetch() as u16
            },
            AddrMode::ZpgX => {
                self.fetch() as u16 + self.registers.x as u16
            },
            AddrMode::ZpgY => {
                self.fetch() as u16 + self.registers.y as u16
            },
            _              => panic!("Attempted to get address without valid addressing mode"),
        }
    }

    // Instruction functions.
    // Type :: Access
    fn ldx(&mut self, mode: AddrMode) {
        let data = self.read_data(mode);
        self.registers.x = data;
    }

    fn ldy(&mut self, mode: AddrMode) {
        let data = self.read_data(mode);
        self.registers.y = data;
    }

    fn lda(&mut self, mode: AddrMode) {
        let data = self.read_data(mode);
        self.registers.a = data;  
    }

    fn stx(&mut self, mode: AddrMode) {
        let data = self.registers.x;
        self.write_data(mode, data);
    }

    fn sty(&mut self, mode: AddrMode) {
        let data = self.registers.y;
        self.write_data(mode, data);
    }

    fn sta(&mut self, mode: AddrMode) {
        let data = self.registers.a;
        self.write_data(mode, data);
    }
}