use crate::cartridge::Cartridge;

pub struct Memory  {
    cartridge: Cartridge,
    wram: Vec<u8>,
    hram: Vec<u8>,
    io: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            cartridge: Cartridge { rom: vec![] },
            wram: vec![],
            hram: vec![],
            io: vec![],
        }
    }
}
