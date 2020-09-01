#![allow(dead_code)]

pub mod cartridge;
pub mod cpu;
pub mod gpu;
pub mod opcodes;
pub mod memory;
pub mod register;

use cpu::Cpu;
use cartridge::Cartridge;

use std::fs::File;

struct Console {
    cpu: Cpu,
    running: bool,
}

// impl Console {
//     fn new() -> Self {
//         Self {
//             cpu: Cpu::new(),
//             running: false,
//         }
//     }

//     fn run(&mut self) {
//         while self.running {
//         //    self.cpu.step()
//         //    self.gpu.step();
//            // Reset cycle counter
//            // CPU step
//            // Check for LCD_ON
//            // GPU step
//            // Check for interrupts
//            // Check for new GPU frame IF LCD is on
//            // Trace if steps is > 400_000
//            // Repeat until self.running == false
//         }
//     }
// }

fn main() {
    // let mut file = File::open("roms/tetris.gb").unwrap();
    let mut file = File::open("roms/06-ld_r_r.gb").unwrap();
    let cartridge = Cartridge::load(&mut file);
    dbg!(cartridge.rom.len());

    let mut cpu = Cpu::new(cartridge);

    for _ in 0..1000000 {
        cpu.step();
    }
}
