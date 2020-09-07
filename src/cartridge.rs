use std::fs::File;
use std::io::prelude::*;

pub trait MBC {
    fn load(&self, address: u16) -> u8;
    fn store(&mut self, address: u16, value: u8);
}

pub struct RomOnly {
    rom: Vec<u8>
}

impl RomOnly {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom
        }
    }
}

impl MBC for RomOnly {
    fn load(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    // Can't write to a RomOnly cartridge
    fn store(&mut self, _address: u16, _value: u8) {}
}

enum MBC1Mode {
    RAM, ROM
}

pub struct MBC1 {
    mode: MBC1Mode,
    rom_bank: u16,
    ram_bank: u16,
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl MBC1 {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            mode: MBC1Mode::ROM,
            rom_bank: 1,
            ram_bank: 0,
            ram: vec![0; 0x800],
            rom,
        }
    }
}

impl MBC for MBC1 {
    fn load(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom[address as usize],
            0x4000..=0x7FFF => {
                let offset = self.rom_bank * 0x4000;
                self.rom[(offset + (address & 0x3FFF)) as usize]
            }
            0xA000..=0xBFFF => 0,
            _ => panic!()
        }
    }

    fn store(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {} // RAM Enable. No op
            0x2000..=0x3FFF => {
                let value = value & 0x1F;
                let value = if value == 0 { 1 } else { value };
                self.rom_bank = self.rom_bank & 0b0110_0000 | value as u16
            }
            0x4000..=0x5FFF => {
                match self.mode {
                    MBC1Mode::ROM => {
                        let offset = if value == 0 { 1 } else { value } as u16;
                        self.rom_bank = self.rom_bank & 0b0001_1111 | (offset << 5);
                    }
                    MBC1Mode::RAM => {
                        self.ram_bank = (value & 3) as u16;
                    }
                };
            }
            0x6000..=0x7FFF => {
                self.mode = if value & 1 == 0 { MBC1Mode::ROM } else { MBC1Mode::RAM }
            }
            0xA000..=0xBFFF => {}
            _ => panic!()
        }
    }
}

#[derive(Debug)]
pub enum CartridgeType {
    RomOnly, // TODO: Add other type (MBC1, MBC3...etc)
    MBC1,
}

pub struct Cartridge {
    pub headers: Headers,
    pub mbc: Box<dyn MBC>,
}

#[derive(Debug)]
pub struct Headers {
    pub cartridge_type: CartridgeType,
    pub rom_size: usize,
    pub ram_size: usize,
}

impl Headers {
    fn cartridge_type(n: u8) -> CartridgeType {
        match n {
            0x00 => CartridgeType::RomOnly,
            0x1 | 0x2 | 0x3 => CartridgeType::MBC1,
            _ => panic!("Unknown cartridge type {}", n),
        }
    }

    fn rom_size(n: u8) -> usize {
        0x8000 << n
    }

    fn ram_size(n: u8) -> usize {
        match n {
            0 => 0,
            1 => 0x800,
            2 => 0x2000,
            3 => 0x8000,
            _ => panic!("Unhandled ram size"),
        }
    }

    fn new(rom: &Vec<u8>) -> Self {
        Self {
            cartridge_type: Headers::cartridge_type(rom[0x147]),
            rom_size: Headers::rom_size(rom[0x148]),
            ram_size: Headers::ram_size(rom[0x149]),
        }
    }
}

impl Cartridge {
    pub fn load(file: &mut File) -> Cartridge {
        // let headers = Headers {
        //     garbage: [0xFF; 100],
        // };

        let mut rom = Vec::new();
        file.read_to_end(&mut rom).expect("Cannot read file");

        let headers = Headers::new(&rom);

        let mbc: Box<dyn MBC> = match headers.cartridge_type {
            CartridgeType::RomOnly => Box::new(RomOnly::new(rom)),
            CartridgeType::MBC1 => Box::new(MBC1::new(rom)),
        };

        Cartridge {
            headers,
            mbc,
        }
    }
}
