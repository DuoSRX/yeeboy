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
        match address {
            0x0000..=0x7FFF => self.rom[address as usize],
            // 0xA000..=0xBFFF => 0,
            _ => panic!(),
        }
    }

    // You can't technically write to the ROM on a real game boy but it's useful in unit tests
    fn store(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => {},
            // 0x0000..=0x7FFF => self.rom[address as usize] = value,
            // 0xA000..=0xBFFF => {},
            _ => panic!("{:04X} {:02X}", address, value),
        }
    }
}

enum MBC1Mode {
    RAM, ROM
}

#[allow(dead_code)]
pub struct MBC1 {
    mode: MBC1Mode,
    rom_bank: u16,
    ram_bank: u16,
    rom: Vec<u8>,
    ram: Vec<u8>, // TODO: Use this
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

// https://gbdev.gg8.se/wiki/articles/MBC1
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

pub struct MBC3 {
    rom_bank: usize,
    ram_bank: usize,
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl MBC3 {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom_bank: 1,
            ram_bank: 0,
            ram: vec![0; 1048576],
            rom,
        }
    }
}

// https://gbdev.gg8.se/wiki/articles/MBC3
impl MBC for MBC3 {
    fn load(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom[address as usize],
            0x4000..=0x7FFF => {
                let offset = self.rom_bank * 0x4000;
                self.rom[(offset + (address as usize & 0x3FFF)) as usize]
            }
            0xA000..=0xBFFF => {
                self.ram[self.ram_bank * 0x2000 + address as usize - 0xA000]
            }
            _ => panic!()
        }
    }

    fn store(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {} // RAM Enable. No op
            0x2000..=0x3FFF => {
                let value = value & 0x7F;
                self.rom_bank = if value == 0 { 1 } else { value } as usize;
            }
            0x4000..=0x5FFF => {
                self.ram_bank = value as usize & 3;
            }
            0x6000..=0x7FFF => {} // TODO: latch clock data
            0xA000..=0xBFFF => {
                self.ram[self.ram_bank * 0x2000 + address as usize - 0xA000] = value;
            }
            _ => panic!()
        }
    }
}

#[derive(Debug)]
pub enum CartridgeType {
    RomOnly,
    MBC1,
    MBC3,
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
            0x01..=0x03 => CartridgeType::MBC1,
            0x0F..=0x13 => CartridgeType::MBC3,
            _ => panic!("Unknown cartridge type {:02X}", n),
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
            CartridgeType::MBC3 => Box::new(MBC3::new(rom)),
        };

        Cartridge {
            headers,
            mbc,
        }
    }
}
