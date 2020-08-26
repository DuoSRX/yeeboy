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
    NOP,
    Sla(Storage),
}

use Instruction::*;

pub fn decode(opcode: u8) -> Instruction {
    match opcode {
        0x00 => NOP,
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
            LdN(r) => { let b = self.load_byte(); self.registers.set(r, b); 8},
            _ => unimplemented!("{:?}", instruction),
        }
    }

    // Load the byte at current PC
    fn load_byte(&mut self) -> u8 {
        self.memory.load(self.pc)
    }

    // Load the byte at current PC and increment PC by 1
    fn load_and_bump_pc(&mut self) -> u8 {
        let byte = self.load_byte();
        self.pc += 1;
        byte
    }
}