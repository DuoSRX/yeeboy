use crate::register::{Registers, Register, Register16, Register::*, Register16::*};
use crate::memory::Memory;

#[derive(Debug)]
pub enum Storage {
    Register(Register),
    Pointer(Register16)
}

use Storage::*;

#[derive(Debug)]
pub enum Instruction {
    Adc(Storage),
    Bit(u8, Storage),
    LdN(Register),
    LdNN(Register16),
    NOP,
    Sla(Storage),
}

use Instruction::*;

pub fn decode(opcode: u8) -> Instruction {
    match opcode {
        0x00 => NOP,
        0x01 => LdNN(BC),
        0x06 => LdN(B),
        0x87 => Adc(Register(B)),
        0x20 => Sla(Register(B)),
        0x46 => Bit(0, Pointer(HL)),
        _ => NOP
    }
}

pub struct Cpu {
    pub pc: u16,
    pub registers: Registers,
    pub memory: Memory,
    pub cycles: u64,
}

impl Cpu {
    pub fn new() -> Self {
        let memory = Memory::new();
        Self {
            registers: Registers::new(),
            pc: 0x100,
            cycles: 0,
            memory,
        }
    }

    // Run an entire CPU step, that is:
    // - Fetch the next opcode at PC
    // - Decode the instruction
    // - Execute the instruction
    // - Increment the cycle count
    pub fn step(&mut self) {
        let opcode = self.load_and_bump_pc();
        let instruction = decode(opcode);
        self.cycles += self.execute(instruction);
    }

    // Execute an instruction and returns the number of cycles taken
    pub fn execute(&mut self, instruction: Instruction) -> u64 {
        match instruction {
            LdN(r) => { let b = self.load_byte(); self.registers.set(r, b); 8 },
            LdNN(r) => { let w = self.load_word(); self.registers.set16(r, w); 12 },
            _ => unimplemented!("{:?}", instruction),
        }
    }

    // Load the byte at current PC
    fn load_byte(&mut self) -> u8 {
        self.memory.load(self.pc)
    }

    // Load the word at current PC
    fn load_word(&mut self) -> u16 {
        self.memory.load16(self.pc)
    }

    // Load the byte at current PC and increment PC by 1
    fn load_and_bump_pc(&mut self) -> u8 {
        let byte = self.load_byte();
        self.pc += 1;
        byte
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn test_load_byte() {
        let mut cpu = Cpu::new();
        cpu.memory.store(cpu.pc, 0x42);
        assert_eq!(cpu.load_byte(), 0x42);
    }

    #[test]
    fn test_load_word() {
        let mut cpu = Cpu::new();
        cpu.memory.store(cpu.pc, 0x42);
        cpu.memory.store(cpu.pc + 1, 0x12);
        assert_eq!(cpu.load_word(), 0x1242);
    }


    #[test]
    fn test_load_and_bump_pc() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x1234;
        cpu.memory.store(cpu.pc, 0x42);
        assert_eq!(cpu.load_and_bump_pc(), 0x42);
        assert_eq!(cpu.pc, 0x1235);
    }

    #[test]
    fn test_ld_n() {
        let mut cpu = Cpu::new();
        cpu.memory.store(cpu.pc, 0x06);
        cpu.memory.store(cpu.pc + 1, 0x42);
        cpu.step();
        assert_eq!(cpu.registers.b, 0x42);
        assert_eq!(cpu.cycles, 8);
    }

    #[test]
    fn test_ld_nn() {
        let mut cpu = Cpu::new();
        cpu.memory.store(cpu.pc, 0x01);
        cpu.memory.store(cpu.pc + 1, 0x34);
        cpu.memory.store(cpu.pc + 2, 0x12);
        cpu.step();
        assert_eq!(cpu.registers.b, 0x12);
        assert_eq!(cpu.registers.c, 0x34);
        assert_eq!(cpu.cycles, 12);
    }
}
