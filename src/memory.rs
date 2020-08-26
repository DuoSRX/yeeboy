use crate::cartridge::Cartridge;
use crate::gpu::Gpu;

pub struct Memory  {
    cartridge: Cartridge, // TODO: Replace with MBC
    work_ram: Vec<u8>,
    high_ram: Vec<u8>,
    gpu: Gpu,
}

impl Memory {
    pub fn new() -> Self {
        let gpu = Gpu::new();
        Self {
            cartridge: Cartridge { rom: vec![] },
            work_ram: vec![0; 0x2000], // 8 kB of RAM
            high_ram: vec![0; 0x7F],   // Mapped from 0xFF80 to 0xFFF
            gpu,
        }
    }

    pub fn load(&mut self, address: u16) -> u8 {
        // match address {
        //     // 0x0000...0x7FFF => rom TODO
        //     0xC000
        // }
        self.work_ram[address as usize]
    }

    pub fn store(&mut self, address: u16, value: u8) {
        self.work_ram[address as usize] = value;
    }

    // Load word at address by loading two consecutive bytes in little endian
    pub fn load16(&mut self, address: u16) -> u16 {
        let lo = self.load(address) as u16;
        let hi = self.load(address + 1) as u16;
        lo | (hi << 8)
    }
}

