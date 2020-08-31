#![allow(dead_code)]

pub const ZERO_FLAG:       u8 = 0b1000_0000;
pub const NEGATIVE_FLAG:        u8 = 0b0100_0000;
pub const HALF_CARRY_FLAG: u8 = 0b0010_0000;
pub const CARRY_FLAG:      u8 = 0b0001_0000;

#[derive(Clone, Copy, Debug)]
pub enum Flag {
    Z, // Zero
    N, // Negative
    H, // Half-carry
    C, // Carry
}

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

// trait RegisterIndex<T> {
//     type Output;
//     fn get1(&self, register: T) -> Self::Output;
//     fn set1(&self, register: T, value: Self::Output);
// }

// impl RegisterIndex<Register8> for Registers {
//     type Output = u8;
//     fn get1(&self, _r: Register8) -> u8 { self.f }
//     fn set1(&self, register: Register8, value: u8) { self.f = value }
// }
// impl RegisterIndex<Register16> for Registers {
//     type Output = u16;
//     fn get1(&self, _r: Register16) -> u16 { self.f as u16 }
//     fn set1(&self, register: Register16, value: u16) { self.f = value as u8 }
// }

// fn foo(r: &Registers) {
//     let a = r.get1(H);
//     let b = r.get1(HL);
// }

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

    pub fn get(&self, register: Register8) -> u8 {
        match register {
            A => self.a,
            B => self.b,
            C => self.c,
            D => self.d,
            E => self.e,
            F => self.f,
            H => self.h,
            L => self.l,
        }
    }

    pub fn set(&mut self, register: Register8, value: u8) {
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

    pub fn get16(&self, register: Register16) -> u16 {
        let (hi, lo) = match register {
            AF => (self.a, self.f),
            BC => (self.b, self.c),
            DE => (self.d, self.e),
            HL => (self.h, self.l),
            SP => { return self.sp }
        };

        ((hi as u16) << 8) | (lo as u16)
    }

    pub fn set16(&mut self, register: Register16, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;
        match register {
            AF => { self.a = hi; self.f = lo }
            BC => { self.b = hi; self.c = lo }
            DE => { self.d = hi; self.e = lo }
            HL => { self.h = hi; self.l = lo }
            SP => self.sp = value
        }
    }

    pub fn flag(&mut self, flag: Flag, cond: bool) {
        if cond {
            self.set_flag(flag);
        } else {
            self.unset_flag(flag);
        }
    }

    pub fn has_flag(&mut self, flag: Flag) -> bool {
        match flag {
            Flag::Z => self.f & ZERO_FLAG > 0,
            Flag::N => self.f & NEGATIVE_FLAG > 0,
            Flag::H => self.f & HALF_CARRY_FLAG > 0,
            Flag::C => self.f & CARRY_FLAG > 0,
        }
    }

    fn set_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Z => self.f |= ZERO_FLAG,
            Flag::N => self.f |= NEGATIVE_FLAG,
            Flag::H => self.f |= HALF_CARRY_FLAG,
            Flag::C => self.f |= CARRY_FLAG,
        }
    }

    fn unset_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Z => self.f &= !ZERO_FLAG & 0xFF,
            Flag::N => self.f &= !NEGATIVE_FLAG & 0xFF,
            Flag::H => self.f &= !HALF_CARRY_FLAG & 0xFF,
            Flag::C => self.f &= !CARRY_FLAG & 0xFF,
        }
    }
}
