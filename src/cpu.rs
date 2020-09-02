use crate::cartridge::Cartridge;
use crate::memory::Memory;
use crate::opcodes::*;
use crate::register::{Flag, Registers, Register8, Register16, Register16::*};

// TODO: Expand this to other types of operation, like nn, d8...etc
// This would cut out drastically on the duplication in some ops
// See for instance: xor d8, xor hl, xor
#[derive(Clone, Copy, Debug)]
pub enum Storage {
    Register(Register8),
    Pointer(Register16),
    NextByte,
}

// impl Storage {
//     fn indirect(&self) -> bool {
//         match self {
//             Storage::Register(_) => false,
//             Storage::Pointer(_)  => true,
//         }
//     }
// }

// TODO: Harmonize the naming convention
// I wouldn't mind having longer name honestly.
// Or even mixing camel and underscores *shudders*
#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Adc(Storage),
    // Bit(u8, Storage),
    AddD8,
    AddHlR16(Register16),
    AndD8,
    Call,
    CallCond(Flag, bool),
    Cp(Storage),
    Cpl,
    Daa,
    Dec(Storage),
    Di,
    Inc(Register8),
    Inc16(Register16),
    Jp,
    JpCond(Flag, bool),
    JpHl,
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
    Or(Storage),
    Pop16(Register16),
    Push16(Register16),
    Ret,
    RetCond(Flag, bool),
    Rr(Storage),
    Rra, // RRA is the same as RR but has a different flag behaviour
    Srl(Register8),
    SubD8,
    Swap(Storage),
    // Sla(Storage),
    Xor(Register8),
    XorD8,
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

    pub fn execute(&mut self, instruction: &Instruction)  {
        match *instruction {
            Adc(s) => {
                let value = self.load(s);
                self.do_adc(value as u16);
            },
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
            AddHlR16(r) => {
                let hl = self.registers.get16(HL) as u32;
                let a = self.registers.get16(r) as u32;
                let result = hl + a;
                let h = (hl & 0xFFF) + (a & 0xFFF) > 0xFFF;
                self.registers.set16(HL, (result & 0xFFFF) as u16);
                self.registers.flag(Flag::H, h);
                self.registers.flag(Flag::N, false);
                self.registers.flag(Flag::C, result > 0xFFFF);
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
            Cp(s) => {
                let byte = self.load(s);
                self.do_cp(byte);
            },
            Cpl => {
                self.registers.a ^= 0xFF;
                self.registers.flag(Flag::H, true);
                self.registers.flag(Flag::N, true);
            },
            Dec(s) => {
                let value = self.load(s);
                let result = value.wrapping_sub(1);
                self.store(s, result);
                self.registers.flag(Flag::H, (result & 0xF) > (value & 0xF));
                self.registers.flag(Flag::Z, result == 0);
                self.registers.flag(Flag::N, true);
            },
            Daa => {
                // WHAT IS EVEN GOING ON HERE OH GOD
                // WHY IS THIS EVEN WORKING
                let n = self.registers.has_flag(Flag::N);
                let h = self.registers.has_flag(Flag::H);
                let c = self.registers.has_flag(Flag::C);
                let a = self.registers.a;
                let mut result = 0;
                if h { result |= 0x06 };
                if c { result |= 0x60 };
                let total = if n {
                    a.wrapping_sub(result)
                } else {
                    if a & 0xF > 9 { result |= 0x06 };
                    if a > 0x99 { result |= 0x60 };
                    a.wrapping_add(result)
                };
                self.registers.a = total;
                self.registers.flag(Flag::H, false);
                self.registers.flag(Flag::Z, total == 0);
                self.registers.flag(Flag::C, result & 0x60 != 0);
            }
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
            Jp => {
                self.pc = self.load_word();
            },
            JpCond(flag, cond) => {
                let address = self.load_word_and_bump_pc();
                if self.registers.has_flag(flag) == cond {
                    self.pc = address;
                    self.cycles += 4;
                }
            },
            JpHl => {
                self.pc = self.registers.get16(HL);
            },
            Jr(flag, cond) => {
                let offset = self.load_and_bump_pc() as i8;
                if self.registers.has_flag(flag) == cond {
                    self.pc = (self.pc as u32 as i32).wrapping_add(offset as i32) as u16;
                    self.cycles += 4;
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
            Or(s) => {
                let value = self.load(s);
                self.do_or(value);
            },
            Pop16(AF) => {
                let af = self.memory.load16(self.registers.sp) & 0xFFF0;
                let sp = self.registers.sp.wrapping_add(2);
                self.registers.set16(AF, af);
                self.registers.set16(SP, sp);
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
                self.store(s, value);
                self.registers.flag(Flag::Z, value == 0);
                self.registers.flag(Flag::N, false);
                self.registers.flag(Flag::H, false);
                self.registers.flag(Flag::C, (a & 1) == 1);
            },
            Rra => {
                let a = self.registers.a;
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
            RetCond(flag, cond) => {
                if self.registers.has_flag(flag) == cond {
                    let sp = self.registers.sp;
                    let pc = self.memory.load16(sp);
                    self.registers.sp = sp.wrapping_add(2);
                    self.pc = pc;
                    self.cycles += 12;
                }
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
            Swap(s) => {
                self.do_swap(s);
            }
            Xor(r) => {
                let result = self.registers.a ^ self.registers.get(r);
                self.registers.a = result;
                self.registers.flag(Flag::Z, result == 0);
                self.registers.flag(Flag::N, false);
                self.registers.flag(Flag::H, false);
                self.registers.flag(Flag::C, false);
            },
            XorD8 => {
                let result = self.registers.a ^ self.load_and_bump_pc();
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
            Storage::NextByte => self.load_and_bump_pc(),
            Storage::Register(r) => self.registers.get(r),
            Storage::Pointer(r) => {
                let address = self.registers.get16(r);
                self.memory.load(address)
            }
        }
    }

    fn store(&mut self, storage: Storage, value: u8) {
        match storage {
            Storage::NextByte => panic!("Can't store at next byte"),
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

    fn do_adc(&mut self, value: u16) {
        let a = self.registers.a as u16;
        let carry = self.registers.has_flag(Flag::C) as u16;
        let result = a + value + carry;
        self.registers.a = (result & 0xFF) as u8;
        let h = (a & 0xF) + (value & 0xF) + carry > 0xF;
        self.registers.flag(Flag::Z, (result & 0xFF) == 0);
        self.registers.flag(Flag::N, false);
        self.registers.flag(Flag::H, h);
        self.registers.flag(Flag::C, result > 0xFF);
    }

    fn do_cp(&mut self, byte: u8) {
        let a = self.registers.a;
        self.registers.flag(Flag::H, (byte & 0xF) > (a & 0xF));
        self.registers.flag(Flag::N, true);
        self.registers.flag(Flag::C, a < byte);
        self.registers.flag(Flag::Z, a == byte);
    }

    fn do_or(&mut self, value: u8) {
        let value = self.registers.a | value;
        self.registers.a = value;
        self.registers.flag(Flag::C, false);
        self.registers.flag(Flag::H, false);
        self.registers.flag(Flag::N, false);
        self.registers.flag(Flag::Z, value == 0);
    }

    fn do_swap(&mut self, s: Storage) {
        let a = self.load(s);
        let lo = (a & 0x0F) << 4;
        let hi = (a & 0xF0) >> 4;
        let result = lo | hi;
        self.store(s, result);
        self.registers.flag(Flag::C, false);
        self.registers.flag(Flag::H, false);
        self.registers.flag(Flag::N, false);
        self.registers.flag(Flag::Z, result == 0);
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
        cpu.registers.b = 0x4;
        cpu.registers.flag(Flag::N, false);
        cpu.memory.store(cpu.pc, 0x05);
        cpu.step();
        assert_eq!(cpu.registers.b, 0x3);
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
        // +2 bytes for the instruction and the operand
        assert_eq!(cpu.pc, 0x100 + 2 - 5);
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
