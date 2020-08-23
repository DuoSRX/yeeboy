use crate::register::Registers;
use crate::memory::Memory;

pub struct Cpu {
    pc: u16,
    registers: Registers,
    // memory: Box<Memory>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            pc: 0x100,
        }
    }
    pub fn step(&mut self, memory: &mut Memory) {}
}