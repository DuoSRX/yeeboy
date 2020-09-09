use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::input::Button;

pub struct Console {
    pub cpu: Cpu,
}

impl Console {
    pub fn new(cartridge: Cartridge, trace: bool) -> Self {
        let cpu = Cpu::new(cartridge, trace);
        Self { cpu }
    }

    pub fn step(&mut self) {
        let prev_cy = self.cpu.cycles;
        self.cpu.step();
        let elapsed = self.cpu.cycles - prev_cy;

        self.cpu.memory.gpu.step(elapsed);

        if self.cpu.memory.timer.tick(elapsed) {
            self.cpu.request_interrupt(4);
        }

        if self.cpu.memory.gpu.interrupts > 0 {
            self.cpu.request_interrupt(self.cpu.memory.gpu.interrupts);
            self.cpu.memory.gpu.interrupts = 0;
        }

        self.cpu.interrupt();
    }

    pub fn new_frame(&self) -> bool {
        self.cpu.memory.gpu.new_frame
    }

    pub fn key_down(&mut self, button: Button) {
        self.cpu.memory.input.key_down(button);
    }

    pub fn key_up(&mut self, button: Button) {
        self.cpu.memory.input.key_up(button);
    }
}
