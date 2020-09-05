#[derive(Default, Debug)]
pub struct Timer {
    counter: u64,
    div_counter: u64,

    pub div: u8,  // FF04
    pub tima: u8, // FF05
    pub tma: u8,  // FF06

    // Bit  2   - Timer Enable
    // Bits 1-0 - Input Clock Select
    //  00: CPU Clock / 1024 (DMG, CGB:   4096 Hz, SGB:   ~4194 Hz)
    //  01: CPU Clock / 16   (DMG, CGB: 262144 Hz, SGB: ~268400 Hz)
    //  10: CPU Clock / 64   (DMG, CGB:  65536 Hz, SGB:  ~67110 Hz)
    //  11: CPU Clock / 256  (DMG, CGB:  16384 Hz, SGB:  ~16780 Hz)
    pub tac: u8,  // FF07
}

impl Timer {
    pub fn new() -> Self {
        Self {
            counter: 0,
            div_counter: 0,
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn tick(&mut self, cycles: u64) -> bool {
        let mut interrupted = false;
        self.div_counter += cycles;

        while self.div_counter >= 256 {
            self.div = self.div.wrapping_add(1);
            self.div_counter -= 256;
        }

        let step = self.clock_select();

        if step > 0 {
            self.counter += cycles;

            while self.counter >= step {
                self.counter -= step;
                self.tima = self.tima.wrapping_add(1);

                // We overflowed, transfer TMA to TIMA // and signal an interrupt
                if self.tima == 0 {
                    self.tima = self.tma;
                    interrupted = true;
                }
            }
        }

        interrupted
    }

    fn clock_select(&self) -> u64 {
        if self.tac & 0x4 == 0 {
            return 0;
        }
        match self.tac & 0x3 {
            1 => 16,
            2 => 64,
            3 => 256,
            _ => 1024,
        }
    }
}