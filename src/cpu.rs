use crate::cartridge::Cartridge;
use crate::memory::Memory;
use crate::register::{Flag, Registers, Register8, Register16, Register8::*, Register16::*};

#[derive(Clone, Copy, Debug)]
pub enum Storage {
    Register(Register8),
    Pointer(Register16)
}

// use Storage::*;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    // Adc(Storage),
    // Bit(u8, Storage),
    Dec(Register8),
    Inc(Register8),
    Inc16(Register16),
    LdN(Register8),
    LdNN(Register16),
    NOP,
    // Sla(Storage),
    NotImplemented,
    Undefined,
}

use Instruction::*;

// Array containing all the instructions indexed by opcode.
// Tuple format: (Instruction, number of cycles, human readable string)
// Does not include the CB instructions which will be stored in a different array.
// Idea: what if instead of the enum, the first item as fn(&mut cpu) -> () ?
static OPCODES: [(Instruction, u64, &'static str); 0x10] = [
    // 0x
    (NOP,            4,  "NOP"),
    (LdNN(BC),       12, "LD BC, nn"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Inc16(BC),      8,  "INC BC"),
    (Inc(B),         4,  "INC B"),
    (Dec(B),         4,  "DEC B"),
    (LdN(B),         8,  "LD B, n"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 1x
    // TODO
];

pub struct Cpu {
    pub pc: u16,
    pub registers: Registers,
    pub memory: Memory,
    pub cycles: u64,
}

impl Cpu {
    pub fn new(cartridge: Cartridge) -> Self {
        let memory = Memory::new(cartridge);
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
        let (instruction, cycles, descr) = Self::decode(opcode);
        self.execute(instruction);
        self.cycles += cycles;
        println!("{}", self.trace(descr));
    }

    // Format the current state of the CPU, registers..etc
    pub fn trace(&mut self, instruction: &'static str) -> String {
        let pc = self.pc - 1;
        format!(
            "AF:{:04X} BC:{:04X} DE:{:04X} HL:{:04X} SP:{:04X} [{}] {:04X}: {:02X} {:02X} {:02X}  {}",
            self.registers.get16(AF),
            self.registers.get16(BC),
            self.registers.get16(DE),
            self.registers.get16(HL),
            self.registers.get16(SP),
            "CHNZ", // TODO: Flags
            pc,
            self.memory.load(pc),
            self.memory.load(pc + 1),
            self.memory.load(pc + 2),
            instruction,
        )
    }

    pub fn decode(opcode: u8) -> (Instruction, u64, &'static str) {
        // TODO: CB opcodes
        OPCODES[opcode as usize]
    }

    // Execute an instruction and returns the number of cycles taken
    pub fn execute(&mut self, instruction: Instruction)  {
        match instruction {
            Dec(r) => {
                let reg = self.registers.get(r);
                let result = reg.wrapping_sub(1);
                self.registers.set(r, result);
                self.registers.flag(Flag::H, (result & 0xF) > (reg & 0xF));
                self.registers.flag(Flag::Z, result == 0);
                self.registers.flag(Flag::N, true);
            },
            Inc(r) => {
                let reg = self.registers.get(r);
                let result = reg.wrapping_add(1);
                self.registers.set(r, result);
                self.registers.flag(Flag::H, (result & 0xF) < (reg & 0xF));
                self.registers.flag(Flag::Z, result == 0);
                self.registers.flag(Flag::N, false);
            },
            Inc16(r) => {
                let result = self.registers.get16(r).wrapping_add(1);
                self.registers.set16(r, result);
            }
            LdN(r) => { let b = self.load_byte(); self.registers.set(r, b); },
            LdNN(r) => { let w = self.load_word(); self.registers.set16(r, w); },
            NOP => {},
            NotImplemented => panic!("Reached unimplemented instruction: {:?} @ {:04X}", instruction, self.pc),
            Undefined => panic!("Executing undefined instruction at {:04X}", self.pc),
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
    use crate::cartridge::Cartridge;
    use crate::register::Flag;

    fn make_cpu() -> Cpu {
        let cart = Cartridge { rom: vec![] };
        Cpu::new(cart)
    }

    #[test]
    fn test_load_byte() {
        let mut cpu = make_cpu();
        cpu.memory.store(cpu.pc, 0x42);
        assert_eq!(cpu.load_byte(), 0x42);
    }

    #[test]
    fn test_load_word() {
        let mut cpu = make_cpu();
        cpu.memory.store(cpu.pc, 0x42);
        cpu.memory.store(cpu.pc + 1, 0x12);
        assert_eq!(cpu.load_word(), 0x1242);
    }


    #[test]
    fn test_load_and_bump_pc() {
        let mut cpu = make_cpu();
        cpu.pc = 0x1234;
        cpu.memory.store(cpu.pc, 0x42);
        assert_eq!(cpu.load_and_bump_pc(), 0x42);
        assert_eq!(cpu.pc, 0x1235);
    }

    #[test]
    fn test_ld_n() {
        let mut cpu = make_cpu();
        cpu.memory.store(cpu.pc, 0x06);
        cpu.memory.store(cpu.pc + 1, 0x42);
        cpu.step();
        assert_eq!(cpu.registers.b, 0x42);
        assert_eq!(cpu.cycles, 8);
    }

    #[test]
    fn test_ld_nn() {
        let mut cpu = make_cpu();
        cpu.memory.store(cpu.pc, 0x01);
        cpu.memory.store(cpu.pc + 1, 0x34);
        cpu.memory.store(cpu.pc + 2, 0x12);
        cpu.step();
        assert_eq!(cpu.registers.b, 0x12);
        assert_eq!(cpu.registers.c, 0x34);
        assert_eq!(cpu.cycles, 12);
    }

    #[test]
    fn test_inc() {
        let mut cpu = make_cpu();
        cpu.registers.b = 0x12;
        cpu.registers.flag(Flag::N, true);
        cpu.memory.store(cpu.pc, 0x04);
        cpu.step();
        assert_eq!(cpu.registers.b, 0x13);
        assert!(!cpu.registers.has_flag(Flag::N));
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_nop() {
        let mut cpu = make_cpu();
        cpu.memory.store(cpu.pc, 0x00);
        cpu.step();
        assert_eq!(cpu.cycles, 4);
    }
}
