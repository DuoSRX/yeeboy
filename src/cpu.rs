use crate::register::Registers;
use crate::memory::Memory;

pub struct Cpu {
  pc: u16,
  registers: Registers,
  memory: Memory,
}

impl Cpu {
}