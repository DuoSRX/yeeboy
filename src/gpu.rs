// use crate::memory::Memory;

#[derive(Debug)]
enum Mode {
    VBlank,
    HBlank,
    OamRead,
    LcdTransfer,
}

use Mode::*;

pub struct Gpu {
    mode: Mode,
    pub cycles: usize,
    pub ly: usize,
    pub lcd: usize,
    pub frame: Vec<u8>,
    pub vram: Vec<u8>,
    interrupts: usize,
    oam: Vec<u8>,
    new_frame: bool,
    rom: Vec<u8>,
}

impl Gpu {
    pub fn new(rom: Vec<u8>) -> Self {
        Gpu {
            mode: HBlank,
            lcd: 0x80,
            cycles: 0,
            ly: 0,
            interrupts: 0,
            frame: vec![0; 160 * 144],
            vram: vec![0; 0x2000],
            oam: vec![0; 0xA0],
            new_frame: false,
            rom
        }
    }

    // TODO;
    // fn load(&self, address: u16) -> u8 { 0 }
    // fn store(&mut self, address: u16, value: u8) {}

    fn set_mode(&mut self, mode: Mode) {
        let cleared = self.lcd & 0b1111_1100;

        match mode {
            HBlank      => self.lcd = cleared,
            VBlank      => self.lcd = cleared + 1,
            OamRead     => self.lcd = cleared + 2,
            LcdTransfer => self.lcd = cleared + 3
        }

        self.mode = mode;
    }

    pub fn step(&mut self, cycles: usize) {
        let cycles = self.cycles + cycles;
        self.interrupts = 0;

        match self.mode {
            OamRead if cycles >= 80 => {
                // TODO: mode = LcdTransfer
            }
            LcdTransfer if cycles >= 172 => {
            }
            HBlank if cycles >= 204 => {
            }
            VBlank if cycles >= 456 => {
            }
            _ => {}
        }
    }
}