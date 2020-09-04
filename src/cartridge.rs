use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub enum CartridgeType {
    RomOnly, // TODO: Add other type (MBC1, MBC3...etc)
    MBC1,
}

#[derive(Debug)]
pub struct Cartridge {
    pub rom: Vec<u8>,
    pub headers: Headers,
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

        Cartridge {
            rom,
            headers,
        }
    }
}
