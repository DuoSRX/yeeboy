#![allow(dead_code)]

pub mod cartridge;
pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod register;

use cpu::Cpu;
use gpu::Gpu;
use memory::Memory;
use register::{Register, Register16, Register::*, Register16::*};

struct Console {
    cpu: Cpu,
    gpu: Gpu,
    memory: Memory,
}

#[derive(Debug)]
enum Storage {
    Register(Register),
    Pointer(Register16)
}

use Storage::*;

#[derive(Debug)]
enum Instruction {
    ADC(Storage),
    BIT(u8, Storage),
    NOP,
    SLA(Storage),
}

use Instruction::*;

fn decode(opcode: u8) -> Instruction {
    match opcode {
        0x00 => NOP,
        0x87 => ADC(Register(B)),
        0x20 => SLA(Register(B)),
        0x46 => BIT(0, Pointer(HL)),
        _ => NOP
    }
}

fn main() {
    // This is used as a shortcut to decode some opcodes because I'm lazy.
    // The order is *important*, do not change!
    // let registers: Vec<Register> = vec![B, C, D, E, H, L, F, A];
    let a = decode(0x46);
    dbg!(a);
}
