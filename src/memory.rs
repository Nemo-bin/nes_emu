pub struct Memory {
    pub ram             : [u8; 0x0800], // 2 KIB Internal Ram
    pub ppu_regs        : [u8; 0x0008], // NES PPU Registers
    pub mirror_ppu_regs : [u8; 0x1FF8], // Mirrors PPU Registers (Repeats every 8 bytes)
    pub apu_io_regs     : [u8; 0x0018], // NES APU / IO Registers
    pub apu_io_func     : [u8; 0x0008], // APU and IO functionality, typically disabled. See CPU Test Mode
    pub cartridge_use   : [u8; 0xBFE0], // Unmapped, available for cartridge use
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            ram             : [0; 0x0800],
            ppu_regs        : [0; 0x0008],
            mirror_ppu_regs : [0; 0x1FF8],
            apu_io_regs     : [0; 0x0018],
            apu_io_func     : [0; 0x0008],
            cartridge_use   : [0; 0xBFE0],   
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x07FF => self.ram[address as usize],
            0x0800..=0x0FFF => self.ram[address as usize - 0x0800],
            0x1000..=0x17FF => self.ram[address as usize - 0x1000],
            0x1800..=0x1FFF => self.ram[address as usize - 0x1800],
            0x2000..=0x2007 => self.ppu_regs[address as usize - 0x2000],
            0x2008..=0x3FFF => self.mirror_ppu_regs[address as usize - 0x2008],
            0x4000..=0x4017 => self.apu_io_regs[address as usize - 0x4000],
            0x4018..=0x401F => self.apu_io_func[address as usize - 0x4018],
            0x4020..=0xFFFF => self.cartridge_use[address as usize - 0x4020],
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x07FF => { self.ram[address as usize]                      = data; },
            0x0800..=0x0FFF => { self.ram[address as usize - 0x0800]             = data; },
            0x1000..=0x17FF => { self.ram[address as usize - 0x1000]             = data; },
            0x1800..=0x1FFF => { self.ram[address as usize - 0x1800]             = data; },
            0x2000..=0x2007 => { self.ppu_regs[address as usize - 0x2000]        = data; },
            0x2008..=0x3FFF => { self.mirror_ppu_regs[address as usize - 0x2008] = data; },
            0x4000..=0x4017 => { self.apu_io_regs[address as usize - 0x4000]     = data; },
            0x4018..=0x401F => { self.apu_io_func[address as usize - 0x4018]     = data; },
            0x4020..=0xFFFF => { self.cartridge_use[address as usize - 0x4020]   = data; },  
        };
    }

    pub fn readWord(&mut self, address: u16) -> u16 {
        let hi_byte = self.read(address) as u16;
        let lo_byte = self.read(address + 1) as u16;
        let data = (hi_byte << 8) | lo_byte;
        return data;
    }
}