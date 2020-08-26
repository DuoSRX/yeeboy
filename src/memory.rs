use crate::cartridge::Cartridge;
use crate::gpu::Gpu;

pub struct Memory  {
    cartridge: Cartridge, // TODO: Replace with MBC
    wram: Vec<u8>,
    hram: Vec<u8>,
    io: Vec<u8>,
    gpu: Gpu,
}

impl Memory {
    pub fn new() -> Self {
        let gpu = Gpu::new();
        Self {
            cartridge: Cartridge { rom: vec![] },
            wram: vec![0xFF; 0xFFFF],
            hram: vec![],
            io: vec![],
            gpu,
        }
    }

    pub fn load(&mut self, address: u16) -> u8 {
        self.wram[address as usize]
    }

    pub fn store(&mut self, address: u16, value: u8) {
        self.wram[address as usize] = value;
    }

    pub fn load16(&mut self, address: u16) -> u16 {
        let lo = self.wram[address as usize] as u16;
        let hi = self.wram[address as usize + 1] as u16;
        lo | (hi << 8)
    }
}

