use crate::cartridge::Cartridge;
use crate::gpu::Gpu;

pub struct Memory  {
    cartridge: Cartridge, // TODO: Replace with MBC
    work_ram: Vec<u8>,
    high_ram: Vec<u8>,
    gpu: Gpu,
}

impl Memory {
    pub fn new(cartridge: Cartridge) -> Self {
        let gpu = Gpu::new();
        Self {
            work_ram: vec![0; 0x2000], // 8 kB of RAM
            high_ram: vec![0; 0x7F],   // Mapped from 0xFF80 to 0xFFF
            gpu,
            cartridge,
        }
    }

    pub fn load(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.rom[address as usize],
            0xC000..=0xDFFF => self.work_ram[(address & 0x1FFF) as usize],
            _ => 0
        }
    }

    pub fn store(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.rom[address as usize] = value,
            0xC000..=0xDFFF => self.work_ram[(address & 0x1FFF) as usize] = value,
            _ => {}
        }
    }

    // Load word at address by loading two consecutive bytes in little endian
    pub fn load16(&mut self, address: u16) -> u16 {
        let lo = self.load(address) as u16;
        let hi = self.load(address + 1) as u16;
        lo | (hi << 8)
    }
}

