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
    CpN,
    Dec(Register8),
    Di,
    Inc(Register8),
    Inc16(Register16),
    Jp,
    Jr(Flag, bool),
    LdAR16(Register16),
    LddHlA,
    LdiAHl,
    LdN(Register8),
    LdNN(Register16),
    LdR16A(Register16),
    LdReadIoN,
    LdRR(Register8, Register8),
    LdSp,
    LdWriteIoN,
    NOP,
    // Sla(Storage),
    Xor(Register8),
    NotImplemented,
    Undefined,
}

use Instruction::*;

// Array containing all the instructions indexed by opcode.
// Tuple format: (Instruction, number of cycles, human readable string)
// Does not include the CB instructions which will be stored in a different array.
// Idea: what if instead of the enum, the first item as fn(&mut cpu) -> () ?
static OPCODES: [(Instruction, u64, &'static str); 0x100] = [
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
    (Inc(C),         4,  "INC C"),
    (Dec(C),         4,  "DEC C"),
    (LdN(C),         8,  "LD C, n"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 1x
    (NotImplemented, 4,  "STOP"),
    (LdNN(DE),       12, "LD DE, nn"),
    (LdR16A(DE),     8, "LD [DE], A"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Inc(D),         4,  "INC D"),
    (Dec(D),         4,  "DEC D"),
    (LdN(D),         8,  "LD D, n"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Inc(E),         4,  "INC E"),
    (Dec(E),         4,  "DEC E"),
    (LdN(E),         8,  "LD E, n"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 2x
    (Jr(Flag::Z, false), 8, "JR NZ, nn"),
    (LdNN(HL),       12, "LD HL, nn"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (LdiAHl,         8,  "LDI A, (HL)"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 3x
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (LdSp,          12,  "LD SP, nn"),
    (LddHlA,         8,  "LDD (HL), A"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (LdN(A),         8,  "LD A, n"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 4x
    (LdRR(B, B),     4,  "LD B, B"),
    (LdRR(B, C),     4,  "LD B, C"),
    (LdRR(B, D),     4,  "LD B, D"),
    (LdRR(B, E),     4,  "LD B, E"),
    (LdRR(B, H),     4,  "LD B, H"),
    (LdRR(B, L),     4,  "LD B, L"),
    (NotImplemented, 4,  "LD B, (HL)"),
    (LdRR(B, A),     4,  "LD B, A"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 5x
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 6x
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 7x
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (LdRR(A, B),     4,  "LD A, B"),
    (LdRR(A, C),     4,  "LD A, C"),
    (LdRR(A, D),     4,  "LD A, D"),
    (LdRR(A, E),     4,  "LD A, E"),
    (LdRR(A, H),     4,  "LD A, H"),
    (LdRR(A, L),     4,  "LD A, L"),
    (NotImplemented, 4,  "LD A, (HL)"),
    (LdRR(A, A),     4,  "LD A, A"),
    // 8x
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // 9x
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // Ax
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Xor(A),         4,  "XOR A"),
    // Bx
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // Cx
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Jp,             16, "JP"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // Dx
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // Ex
    (LdWriteIoN,     12, "LDH (FF00+n), A"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    // Fx
    (LdReadIoN,      12, "LDH A, (FF00+n)"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (Di,             4,  "DI"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
    (CpN,            8,  "CP n"),
    (NotImplemented, 4,  "NOT IMPLEMENTED YET"),
];

pub struct Cpu {
    pub pc: u16,
    pub registers: Registers,
    pub memory: Memory,
    pub cycles: u64,
    pub ime: bool,
}

impl Cpu {
    pub fn new(cartridge: Cartridge) -> Self {
        let memory = Memory::new(cartridge);
        Self {
            registers: Registers::new(),
            pc: 0x100,
            cycles: 0,
            ime: true,
            memory,
        }
    }

    // Run an entire CPU step, that is:
    // - Fetch the next opcode at PC
    // - Decode the instruction
    // - Execute the instruction
    // - Increment the cycle count
    pub fn step(&mut self) {
        let opcode = self.load_byte();
        let (instruction, cycles, descr) = Self::decode(opcode);
        println!("{}", self.trace(descr));
        self.pc += 1;
        self.execute(instruction);
        self.cycles += cycles;
    }

    // Format the current state of the CPU, registers..etc
    pub fn trace(&mut self, instruction: &'static str) -> String {
        let flags = vec![Flag::C, Flag::H, Flag::N, Flag::Z].iter().map(|&f|
            if self.registers.has_flag(f) { format!("{:?}", f) } else { "-".into() }
        ).collect::<Vec<String>>().join("");

        format!(
            "AF:{:04X} BC:{:04X} DE:{:04X} HL:{:04X} SP:{:04X} [{}] {:04X}: {:02X} {:02X} {:02X}  {}",
            self.registers.get16(AF),
            self.registers.get16(BC),
            self.registers.get16(DE),
            self.registers.get16(HL),
            self.registers.get16(SP),
            flags,
            self.pc,
            self.memory.load(self.pc),
            self.memory.load(self.pc + 1),
            self.memory.load(self.pc + 2),
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
            CpN => {
                let a = self.registers.a;
                let byte = self.load_and_bump_pc();
                self.registers.flag(Flag::H, (byte & 0xF) > (a & 0xF));
                self.registers.flag(Flag::N, true);
                self.registers.flag(Flag::C, a < byte);
                self.registers.flag(Flag::Z, a == byte);
            },
            Dec(r) => {
                let reg = self.registers.get(r);
                let result = reg.wrapping_sub(1);
                self.registers.set(r, result);
                self.registers.flag(Flag::H, (result & 0xF) > (reg & 0xF));
                self.registers.flag(Flag::Z, result == 0);
                self.registers.flag(Flag::N, true);
            },
            Di => self.ime = false,
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
            Jp => { let dest = self.load_word(); self.pc = dest; },
            Jr(flag, cond) => {
                if self.registers.has_flag(flag) == cond {
                    let offset = self.load_and_bump_pc() as i8;
                    self.pc = (self.pc as u32 as i32).wrapping_add(offset as i32) as u16;
                    self.cycles += 4;
                } else {
                    self.pc += 1;
                }
            },
            LddHlA => {
                let address = self.registers.get16(HL);
                self.memory.store(address, self.registers.a);
                self.registers.set16(HL, address.wrapping_sub(1));
            },
            LdiAHl => {
                let address = self.registers.get16(HL);
                self.registers.a = self.memory.load(address);
                self.registers.set16(HL, address.wrapping_add(1));
            },
            LdN(r) => {
                let b = self.load_and_bump_pc();
                self.registers.set(r, b);
            },
            LdNN(r) => {
                let w = self.load_word_and_bump_pc();
                self.registers.set16(r, w);
            },
            LdAR16(r) => {
                let address = self.registers.get16(r);
                self.registers.a = self.memory.load(address);
            },
            LdR16A(r) => {
                let address = self.registers.get16(r);
                self.memory.store(address, self.registers.a);
            },
            LdReadIoN => {
                let n = self.load_and_bump_pc() as u16;
                let byte = self.memory.load(n.wrapping_add(0xFF00));
                self.registers.a = byte;
            },
            LdRR(r1, r2) => {
                self.registers.set(r1, self.registers.get(r2))
            },
            LdSp => {
                self.registers.sp = self.load_word_and_bump_pc();
            },
            LdWriteIoN => {
                let n = self.load_and_bump_pc() as u16;
                let address = n.wrapping_add(0xFF00);
                self.memory.store(address, self.registers.a);
            },
            Xor(r) => {
                let result = self.registers.a ^ self.registers.get(r);
                self.registers.a = result;
                self.registers.flag(Flag::Z, result == 0);
                self.registers.flag(Flag::N, false);
                self.registers.flag(Flag::H, false);
                self.registers.flag(Flag::C, false);
            },
            NOP => {},
            NotImplemented => {
                let opcode = self.memory.load(self.pc - 1);
                panic!("Reached unimplemented instruction: Opcode {:02X} @ {:04X}", opcode, self.pc)
            },
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

    fn load_word_and_bump_pc(&mut self) -> u16 {
        let word = self.load_word();
        self.pc += 2;
        word
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;
    use crate::cartridge::Cartridge;
    use crate::register::Flag;

    fn make_cpu() -> Cpu {
        let cart = Cartridge { rom: vec![0; 0x2000] };
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
        assert_eq!(cpu.pc, 0x102);
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
        assert_eq!(cpu.pc, 0x103);
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
    fn test_ld_rr() {
        let mut cpu = make_cpu();
        cpu.registers.a = 0x43;
        cpu.registers.b = 0x12;
        cpu.memory.store(cpu.pc, 0x47);
        cpu.step();
        assert_eq!(cpu.registers.b, 0x43);
    }

    #[test]
    fn test_dec() {
        let mut cpu = make_cpu();
        cpu.registers.b = 0x12;
        cpu.registers.flag(Flag::N, false);
        cpu.memory.store(cpu.pc, 0x05);
        cpu.step();
        assert_eq!(cpu.registers.b, 0x11);
        assert!(cpu.registers.has_flag(Flag::N));
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn test_jp() {
        let mut cpu = make_cpu();
        cpu.memory.store(cpu.pc, 0xC3);
        cpu.memory.store16(cpu.pc + 1, 0x1234);
        cpu.step();
        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(cpu.cycles, 16);
    }

    #[test]
    fn jr() {
        let mut cpu = make_cpu();
        cpu.memory.store(cpu.pc, 0x20);
        cpu.memory.store(cpu.pc + 1, -5 as i8 as u8);
        cpu.registers.flag(Flag::Z, false);
        cpu.step();
        assert_eq!(cpu.pc, 0xFB);
        assert_eq!(cpu.cycles, 12);

        let mut cpu = make_cpu();
        cpu.memory.store(cpu.pc, 0x20);
        cpu.registers.flag(Flag::Z, true);
        cpu.step();
        assert_eq!(cpu.pc, 0x102);
        assert_eq!(cpu.cycles, 8);
    }

    #[test]
    fn test_nop() {
        let mut cpu = make_cpu();
        cpu.memory.store(cpu.pc, 0x00);
        cpu.step();
        assert_eq!(cpu.cycles, 4);
    }
}
