#![allow(dead_code)]

pub mod cartridge;
pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod register;

use cpu::Cpu;

struct Console {
    cpu: Cpu,
    running: bool,
}

impl Console {
    fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            running: false,
        }
    }

    fn run(&mut self) {
        while self.running {
        //    self.cpu.step()
        //    self.gpu.step();
           // Reset cycle counter
           // CPU step
           // Check for LCD_ON
           // GPU step
           // Check for interrupts
           // Check for new GPU frame IF LCD is on
           // Trace if steps is > 400_000
           // Repeat until self.running == false
        }
    }
}

fn main() {
    // This is used as a shortcut to decode some opcodes because I'm lazy.
    // The order is *important*, do not change!
    // let registers: Vec<Register> = vec![B, C, D, E, H, L, F, A];
    let mut cpu = Cpu::new();
    dbg!(cpu.registers.b);
    cpu.memory.store(cpu.pc, 0x06);
    cpu.memory.store(cpu.pc + 1, 0x10);
    cpu.step();
    dbg!(cpu.registers.b);
}
