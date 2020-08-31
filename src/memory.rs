use crate::cartridge::Cartridge;
use crate::gpu::Gpu;

pub struct Memory  {
    cartridge: Cartridge, // TODO: Replace with MBC
    work_ram: Vec<u8>,
    high_ram: Vec<u8>,
    io: Vec<u8>,
    gpu: Gpu,
}

impl Memory {
    pub fn new(cartridge: Cartridge) -> Self {
        let gpu = Gpu::new();
        Self {
            work_ram: vec![0; 0x2000], // 8 kB of RAM
            high_ram: vec![0; 0x80],   // Mapped from 0xFF80 to 0xFFF
            io: vec![0; 0x80],
            gpu,
            cartridge,
        }
    }

    pub fn load(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.rom[address as usize],
            0xC000..=0xDFFF => self.work_ram[(address & 0x1FFF) as usize],
            0xFF00..=0xFF7F => self.io[address as usize - 0xFF00],
            0xFF80..=0xFFFF => self.io[address as usize - 0xFF80],
            _ => unimplemented!()
        }
    }

    pub fn store(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.rom[address as usize] = value,
            0xC000..=0xDFFF => self.work_ram[(address & 0x1FFF) as usize] = value,
            0xFF00..=0xFF7F => self.io[address as usize - 0xFF00] = value,
            0xFF80..=0xFFFF => self.io[address as usize - 0xFF80] = value,
            _ => unimplemented!()
        }
    }

    // Load word at address by loading two consecutive bytes in little endian
    pub fn load16(&mut self, address: u16) -> u16 {
        let lo = self.load(address) as u16;
        let hi = self.load(address + 1) as u16;
        lo | (hi << 8)
    }

    pub fn store16(&mut self, address: u16, value: u16) {
        let lo = value & 0xFF;
        let hi = (value & 0xFF00) >> 8;
        self.store(address, lo as u8);
        self.store(address + 1, hi as u8);
    }
}

