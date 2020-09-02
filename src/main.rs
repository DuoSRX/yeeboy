#![allow(dead_code)]

pub mod cartridge;
pub mod cpu;
pub mod gpu;
pub mod opcodes;
pub mod memory;
pub mod register;

use cpu::Cpu;
use cartridge::Cartridge;
// use memory::Memory;

use std::fs::File;

// struct Console {
//     cpu: Cpu,
//     running: bool,
// }

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
    // let mut file = File::open("roms/drmario.gb").unwrap();
    // let mut file = File::open("roms/01-special.gb").unwrap();
    // let mut file = File::open("roms/03-op_sp_hl.gb").unwrap();
    // let mut file = File::open("roms/04-op_r_imm.gb").unwrap();
    // let mut file = File::open("roms/05-op_rp.gb").unwrap();
    // let mut file = File::open("roms/06-ld_r_r.gb").unwrap();
    let mut file = File::open("roms/07-jr_jp_call_ret_rst.gb").unwrap();
    // let mut file = File::open("roms/08-misc_instrs.gb").unwrap();
    // let mut file = File::open("roms/09-op_r_r.gb").unwrap();
    let cartridge = Cartridge::load(&mut file);
    dbg!(cartridge.rom.len());

    let mut cpu = Cpu::new(cartridge);

    // for _ in 0..500000 {
        // cpu.step();
    // }

    loop {
        let prev_cy = cpu.cycles;
        cpu.step();

        let lcd_on = cpu.memory.load(0xFF40) & 0x80 > 0;
        cpu.memory.gpu.step(cpu.cycles - prev_cy, lcd_on);

        if cpu.memory.gpu.interrupts > 0 {
            // TODO: Gpu Interrupts
        }

        // TODO: Cpu interrupts
        // TODO: Check for new frame & lcd_on
        if cpu.memory.gpu.new_frame && lcd_on {
            // dbg!(&cpu.memory.gpu.oam);
            cpu.memory.gpu.new_frame = false;
        }

        if !cpu.memory.serial.is_empty() {
            print!("{}", cpu.memory.serial.remove(0));
        }
    }
}
