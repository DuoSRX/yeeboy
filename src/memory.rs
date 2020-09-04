use crate::cartridge::Cartridge;
use crate::timer::Timer;
use crate::gpu::Gpu;

pub struct Memory  {
    cartridge: Cartridge, // TODO: Replace with MBC
    work_ram: Vec<u8>,
    high_ram: Vec<u8>,
    io: Vec<u8>,
    pub serial: Vec<char>, // for debugging only
    pub gpu: Gpu,
    pub timer: Timer,
}

impl Memory {
    pub fn new(cartridge: Cartridge) -> Self {
        // TODO: Share a reference instead of cloning the entire rom
        let gpu = Gpu::new(cartridge.rom.clone());
        Self {
            work_ram: vec![0; 0x2000], // 8 kB of RAM
            high_ram: vec![0; 0x80],   // Mapped from 0xFF80 to 0xFFF
            io: vec![0; 0x80],
            serial: vec![],
            timer: Timer::new(),
            cartridge,
            gpu,
        }
    }

    pub fn load(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.rom[address as usize],
            0x8000..=0x9FFF => self.gpu.load(address),
            0xC000..=0xDFFF => self.work_ram[(address & 0x1FFF) as usize],
            0xE000..=0xFDFF => self.work_ram[((address - 0x2000) & 0x1FFF) as usize],
            0xFE00..=0xFE9F => 0, // TODO: OAM
            0xFEA0..=0xFEFF => 0, // No-op
            0xFF04 => self.timer.div,
            0xFF05 => self.timer.tima,
            0xFF06 => self.timer.tma,
            0xFF07 => self.timer.tac,
            0xFF40 => self.gpu.control,
            0xFF42 => self.gpu.scroll_y,
            0xFF43 => self.gpu.scroll_x,
            0xFF44 => self.gpu.ly,
            0xFF47 => self.gpu.bg_palette,
            0xFF48 => self.gpu.obj_palette_0,
            0xFF49 => self.gpu.obj_palette_1,
            0xFF00..=0xFF7F => self.io[address as usize - 0xFF00],
            0xFF80..=0xFFFF => self.high_ram[address as usize - 0xFF80],
            _ => unimplemented!("Loading {:04X}", address),
        }
    }

    pub fn store(&mut self, address: u16, value: u8) {
        if address == 0xFF01 {
            unsafe { self.serial.push(std::char::from_u32_unchecked(value as u32)) }
            // unsafe { dbg!(std::char::from_u32_unchecked(value as u32)) };
            return
        }
        match address {
            // 0x0000..=0x7FFF => self.cartridge.rom[address as usize] = value,
            0x0000..=0x7FFF => {} // TODO: Cartridge ram, MBC...etc
            0x8000..=0x9FFF => self.gpu.store(address, value),
            0xC000..=0xDFFF => self.work_ram[(address & 0x1FFF) as usize] = value,
            0xE000..=0xFDFF => self.work_ram[((address - 0x2000) & 0x1FFF) as usize] = value,
            0xFE00..=0xFE9F => self.gpu.oam_store(address - 0xFE00, value),
            0xFEA0..=0xFEFF => {} // No-op
            0xFF04 => self.timer.div = value,
            0xFF05 => self.timer.tima = value,
            0xFF06 => self.timer.tma = value,
            0xFF07 => self.timer.tac = value,
            0xFF40 => self.gpu.control = value,
            0xFF42 => self.gpu.scroll_y = value,
            0xFF43 => self.gpu.scroll_x = value,
            0xFF44 => self.gpu.ly = 0,
            0xFF46 => { // OAM DMA
                let start = (value as u16) << 8;
                for offset in 0..0xA0 {
                    let byte = self.load(start + offset);
                    self.gpu.oam_store(offset, byte);
                }
            }
            0xFF47 => self.gpu.bg_palette = value,
            0xFF48 => self.gpu.obj_palette_0 = value,
            0xFF49 => self.gpu.obj_palette_1 = value,
            0xFF00..=0xFF7F => self.io[address as usize - 0xFF00] = value,
            0xFF80..=0xFFFF => self.high_ram[address as usize - 0xFF80] = value,
            _ => unimplemented!("Storing {:02X} @ {:04X}", value, address),
        }
    }

    // Load word at address by loading two consecutive bytes in little endian
    pub fn load16(&mut self, address: u16) -> u16 {
        let lo = self.load(address) as u16;
        let hi = self.load(address + 1) as u16;
        lo | (hi << 8)
    }

    pub fn store16(&mut self, address: u16, value: u16) {
        let lo = value & 0xFF;
        let hi = (value & 0xFF00) >> 8;
        self.store(address, lo as u8);
        self.store(address + 1, hi as u8);
    }
}

