#![allow(dead_code)]

#[derive(Clone, Copy, Debug)]
pub enum Register8 {
    A, B, C, D, E, F, H, L
}

#[derive(Clone, Copy, Debug)]
pub enum Register16 {
    AF, BC, DE, HL, SP
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
}

use Register8::*;
use Register16::*;

impl Registers {
    pub fn new() -> Self {
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

    pub fn set8(&mut self, register: Register8, value: u8) {
        match register {
            A => self.a = value,
            B => self.b = value,
            C => self.c = value,
            D => self.d = value,
            E => self.e = value,
            F => self.f = value,
            H => self.h = value,
            L => self.l = value,
        }
    }

    pub fn set16(&mut self, register: Register16, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;
        match register {
            AF => { self.a = hi; self.f = lo }
            BC => { self.b = hi; self.c = lo }
            DE => { self.b = hi; self.c = lo }
            HL => { self.b = hi; self.c = lo }
            SP => self.sp = value
        }
    }
}