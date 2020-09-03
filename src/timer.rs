#[derive(Default, Debug)]
pub struct Timer {
    cycles: u64,
    counter: usize,
    div_counter: usize,

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
        Default::default()
    }

    pub fn tick(&mut self, cycles: u64) -> bool {
        self.cycles += cycles;

        // The CPU runs at around 4 MHz and div increments at 16 MHz
        // So we only run the process every 4 CPU cycles
        if self.cycles >= 4 {
            self.cycles -= 4;
            self.counter += 1;
            self.div_counter += 1;
            if self.div_counter == 16 {
                self.div = self.div.wrapping_add(1);
                self.div_counter = 0;
            }
        }

        let cs = self.clock_select();
        if cs > 0 && self.counter > cs {
            self.counter = 0;
            self.tima = self.tima.wrapping_add(1);
            if self.tima == 0 {
                self.tima = self.tma;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn clock_select(&self) -> usize {
        if self.tac & 0x4 == 0 {
            return 0;
        };
        match self.tac & 0x3 {
            0 => 1024,
            1 => 16,
            2 => 64,
            _ => 256,
        }
    }
}