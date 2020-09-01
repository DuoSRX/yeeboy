use crate::cartridge::Cartridge;
use crate::memory::Memory;
use crate::opcodes::*;
use crate::register::{Flag, Registers, Register8, Register16, Register16::*};

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
    AddD8,
    AndD8,
    Call,
    CallCond(Flag, bool),
    CpN,
    Dec(Register8),
    Di,
    Inc(Register8),
    Inc16(Register16),
    Jp,
    Jr(Flag, bool),
    JrE8,
    Lda16A,
    LdAA16,
    LdAR16(Register16),
    LddHlA,
    LdHlR(Register8),
    LdRHl(Register8),
    LdiAHl,
    LdiHlA,
    LdN(Register8),
    LdNN(Register16),
    LdR16A(Register16),
    LdReadIoN,
    LdRR(Register8, Register8),
    LdSp,
    LdWriteIoN,
    NOP,
    Or(Register8),
    Pop16(Register16),
    Push16(Register16),
    Ret,
    Rr(Storage),
    Srl(Register8),
    SubD8,
    // Sla(Storage),
    Xor(Register8),
    XorHl,
    NotImplemented,
    Undefined,
}

use Instruction::*;

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
        // TODO: Fix this ugly duplication. Too lazy right now
        let cycles = match self.load_byte() {
            0xCB => {
                let opcode = self.memory.load(self.pc + 1);
                let (instruction, cycles, descr) = Self::decode_cb(opcode);
                println!("{}", self.trace(descr));
                self.pc += 2;
                self.execute(instruction);
                cycles
            }
            opcode => {
                let (instruction, cycles, descr) = Self::decode(opcode);
                println!("{}", self.trace(descr));
                self.pc += 1;
                self.execute(instruction);
                cycles
            }
        };

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

    pub fn decode(opcode: u8) -> &'static (Instruction, u64, &'static str) {
        OPCODES.get(opcode as usize).unwrap()
    }

    pub fn decode_cb(opcode: u8) -> &'static (Instruction, u64, &'static str) {
        CB_OPCODES.get(opcode as usize).unwrap()
    }

    // Execute an instruction and returns the number of cycles taken
    pub fn execute(&mut self, instruction: &Instruction)  {
        match *instruction {
            AddD8 => {
                let b = self.load_and_bump_pc();
                let value = self.registers.a.wrapping_add(b);
                self.registers.a = value;
                self.registers.flag(Flag::H, (value & 0xF) < (b & 0xF));
                self.registers.flag(Flag::N, false);
                self.registers.flag(Flag::C, value < b);
                self.registers.flag(Flag::Z, value == 0);
            },
            AndD8 => {
                let value = self.registers.a & self.load_and_bump_pc();
                self.registers.a = value;
                self.registers.flag(Flag::H, true);
                self.registers.flag(Flag::N, false);
                self.registers.flag(Flag::C, false);
                self.registers.flag(Flag::Z, value == 0);
            },
            Call => {
                self.do_call();
            },
            CallCond(flag, cond) => {
                if self.registers.has_flag(flag) == cond {
                    self.do_call();
                } else {
                    self.pc += 2;
                }
            },
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
            JrE8 => {
                let offset = self.load_and_bump_pc() as i8;
                self.pc = (self.pc as u32 as i32).wrapping_add(offset as i32) as u16;
            },
            Lda16A => {
                let address = self.load_word_and_bump_pc();
                self.memory.store(address, self.registers.a);
            },
            LdAA16 => {
                let address = self.load_word_and_bump_pc();
                self.registers.a = self.memory.load(address);
            },
            LddHlA => {
                let address = self.registers.get16(HL);
                self.memory.store(address, self.registers.a);
                self.registers.set16(HL, address.wrapping_sub(1));
            },
            LdHlR(r) => {
                let address = self.registers.get16(HL);
                self.memory.store(address, self.registers.get(r));
            },
            LdRHl(r) => {
                let address = self.registers.get16(HL);
                self.registers.set(r, self.memory.load(address));
            },
            LdiAHl => {
                let address = self.registers.get16(HL);
                self.registers.a = self.memory.load(address);
                self.registers.set16(HL, address.wrapping_add(1));
            },
            LdiHlA => {
                let address = self.registers.get16(HL);
                self.memory.store(address, self.registers.a);
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
            Or(r) => {
                let value = self.registers.a | self.registers.get(r);
                self.registers.a = value;
                self.registers.flag(Flag::C, false);
                self.registers.flag(Flag::H, false);
                self.registers.flag(Flag::N, false);
                self.registers.flag(Flag::Z, value == 0);
            },
            Pop16(AF) => {
                let af = self.memory.load16(self.registers.sp) & 0xFFF0;
                let sp = self.registers.sp.wrapping_add(2);
                self.registers.set16(AF, af);
                self.registers.sp = sp;
                self.registers.flag(Flag::C, af & 0x10 > 0);
                self.registers.flag(Flag::H, af & 0x20 > 0);
                self.registers.flag(Flag::N, af & 0x40 > 0);
                self.registers.flag(Flag::Z, af & 0x80 > 0);
            },
            Pop16(r) => {
                let value = self.memory.load16(self.registers.sp);
                let sp = self.registers.sp.wrapping_add(2);
                self.registers.set16(r, value);
                self.registers.sp = sp;
            },
            Push16(r) => {
                let sp = self.registers.sp.wrapping_sub(2);
                self.memory.store16(sp, self.registers.get16(r));
                self.registers.sp = sp;
            },
            Rr(s) => {
                let a = self.load(s);
                let carry = self.registers.has_flag(Flag::C) as u8;
                let value = (carry << 7) | (a >> 1);
                self.registers.a = value;
                self.registers.flag(Flag::Z, false);
                self.registers.flag(Flag::N, false);
                self.registers.flag(Flag::H, false);
                self.registers.flag(Flag::C, (a & 1) == 1);
            },
            Ret => {
                let sp = self.registers.sp;
                let pc = self.memory.load16(sp);
                self.registers.sp = sp.wrapping_add(2);
                self.pc = pc;
            },
            Srl(r) => {
                let a = self.registers.get(r);
                let result = a >> 1;
                self.registers.set(r, result);
                self.registers.flag(Flag::Z, result == 0);
                self.registers.flag(Flag::N, false);
                self.registers.flag(Flag::H, false);
                self.registers.flag(Flag::C, a & 1 == 1);
            },
            SubD8 => {
                let a = self.registers.a;
                let b = self.load_and_bump_pc();
                let value = a.wrapping_sub(b);
                self.registers.a = value;
                self.registers.flag(Flag::H, (value & 0xF) > (a & 0xF));
                self.registers.flag(Flag::N, true);
                self.registers.flag(Flag::C, value > a);
                self.registers.flag(Flag::Z, a == b);
            },
            Xor(r) => {
                let result = self.registers.a ^ self.registers.get(r);
                self.registers.a = result;
                self.registers.flag(Flag::Z, result == 0);
                self.registers.flag(Flag::N, false);
                self.registers.flag(Flag::H, false);
                self.registers.flag(Flag::C, false);
            },
            XorHl => {
                let b = self.memory.load(self.registers.get16(HL));
                let result = self.registers.a ^ b;
                self.registers.a = result;
                self.registers.flag(Flag::Z, result == 0);
                self.registers.flag(Flag::N, false);
                self.registers.flag(Flag::H, false);
                self.registers.flag(Flag::C, false);
            },
            NOP => {},
            Undefined => panic!("Executing undefined instruction at {:04X}", self.pc),
            _ => {
                let opcode = self.memory.load(self.pc - 1);
                panic!("Reached unimplemented instruction: Opcode {:02X} @ {:04X}", opcode, self.pc)
            },
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

    fn load(&mut self, storage: Storage) -> u8 {
        match storage {
            Storage::Register(r) => self.registers.get(r),
            Storage::Pointer(r) => {
                let address = self.registers.get16(r);
                self.memory.load(address)
            }
        }
    }

    fn store(&mut self, storage: Storage, value: u8) {
        match storage {
            Storage::Register(r) => {
                self.registers.set(r, value);
            }
            Storage::Pointer(r) => {
                let address = self.registers.get16(r);
                self.memory.store(address, value)
            }
        }
    }

    fn do_call(&mut self) {
        let address = self.load_word();
        let sp = self.registers.sp.wrapping_sub(2);
        self.memory.store16(sp, self.pc + 2);
        self.registers.sp = sp;
        self.pc = address;
        self.cycles += 12;
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
