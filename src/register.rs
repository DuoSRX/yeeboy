#![allow(dead_code)]

#[derive(Debug)]
pub enum Register {
    A, B, C, D, E, F, H, L
}

#[derive(Debug)]
pub enum Register16 {
    AF, BC, DE, HL, SP
}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
}

impl Registers {
    fn new() -> Self {
        Self {
            a: 1,
            b: 0,
            c: 0x13,
            d: 0,
            e: 0xD8,
            f: 0xB0,
            h: 1,
            l: 0x4D,
            sp: 0xFFFE,
        }
    }
}