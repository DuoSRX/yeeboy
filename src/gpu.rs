// use crate::memory::Memory;

#[derive(Debug)]
enum Mode {
    VBlank,
    HBlank,
    OamRead,
    LcdTransfer,
}

use Mode::*;

pub struct Gpu {
    rom: Vec<u8>,
    mode: Mode,
    pub cycles: u64,
    pub ly: u8,
    pub lcd: u8,
    pub frame: Vec<u8>,
    pub vram: Vec<u8>,
    pub interrupts: usize,
    pub oam: Vec<Sprite>,
    pub new_frame: bool,
    pub control: u8,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub bg_palette: u8,
}

impl Gpu {
    pub fn new(rom: Vec<u8>) -> Self {
        Gpu {
            mode: HBlank,
            lcd: 0x80,
            cycles: 0,
            ly: 0,
            scroll_x: 0,
            scroll_y: 0,
            control: 0,
            interrupts: 0,
            bg_palette: 0,
            frame: vec![0xFF; 160 * 144 * 3],
            vram: vec![0; 0x2000],
            oam: vec![Sprite::new(); 0x40],
            new_frame: false,
            rom
        }
    }

    pub fn step(&mut self, cycles: u64, lcd_on: bool) {
        self.cycles += cycles;
        self.interrupts = 0;

        match self.mode {
            OamRead if self.cycles >= 80 => {
                self.cycles -= 80;
                self.set_mode(LcdTransfer)
            }
            LcdTransfer if self.cycles >= 172 => {
                self.cycles -= 172;
                if lcd_on {
                    if self.control & 1 == 1 {
                        self.render_background();
                    } else {
                        self.clear_frame();
                    }
                    // TODO: render window
                    // self.render_sprites();
                }
                self.set_mode(HBlank)
            }
            HBlank if self.cycles >= 204 => {
                self.cycles -= 204;
                self.ly += 1;
                if self.ly == 144 {
                    self.interrupts = 1;
                    self.new_frame = true;
                    self.set_mode(VBlank);
                } else {
                    self.set_mode(OamRead);
                }
            }
            VBlank if self.cycles >= 456 => {
                self.cycles -= 456;
                self.ly += 1;
                if self.ly >= 154 {
                    self.ly = 0;
                    self.set_mode(OamRead);
                }
            }
            _ => {}
        }

        // TODO: Compare ly and lyc and fire interrupt
    }

    fn render_background(&mut self) {
        let palette = self.bg_palette;
        let colors = [
            palette & 3,
            (palette >> 2) & 3,
            (palette >> 4) & 3,
            (palette >> 6) & 3,
        ];

        // http://bgb.bircd.org/pandocs.htm#lcdpositionandscrolling
        let ly = self.ly as u16;
        let scroll_x = self.scroll_x as u16;
        let scroll_y = self.scroll_y as u16;
        let tile_data: u16 = 0x8000; // TODO: Check control 0x10
        let tile_map: u16 = 0x9800; // TODO: Check control 0x08
        let y: u16 = ((scroll_y + ly) / 8) % 32;
        let y_offset = (scroll_y + ly) % 8;

        for px in 0..160 {
            let x = ((scroll_x + px) / 8) % 32;
            let tile = self.load(tile_map.wrapping_add(y * 32).wrapping_add(x));
            // Handle > 0x9000
            let ptr = tile_data.wrapping_add(tile as u16 * 0x10);
            let ptr = ptr.wrapping_add(y_offset * 2);
            let p0 = self.load(ptr);
            let p1 = self.load(ptr + 1);
            let colb = -(((px + scroll_x) as i32 % 8) - 7);
            let coln = if (p1 >> colb) & 1 == 1 { 1 } else { 0 };
            let coln = (coln << 1) | (if (p0 >> colb) & 1 == 1 { 1 } else { 0 });
            let color = colors[coln as usize];
            self.set_pixel(px as u8, ly as u8, color);
        }
    }

    fn render_sprites(&mut self) {
        let sprite_height = 8; // TODO: 16 px sprites
        let ly = self.ly as i16;

        let mut n = 0;
        while n < 156 {
            let sprite = self.oam[n / 4];

            let y = sprite.y as i16 - 16;
            let x = sprite.x as i16 - 8;
            let on_scanline = (y <= ly) && (y + sprite_height) > ly;

            if on_scanline {
                let y_offset = ly - y; // TODO: y-flip
                let ptr = ((sprite.index * 16) + ((y_offset as u8) * 2)) as u16;
                let lo = self.load(0x8000 + ptr);
                let hi = self.load(0x8000 + ptr + 1);

                for idx_x in 0..8 { // FIXME: check if it's inclusive
                    let pixel_x = x + idx_x;
                    if pixel_x >= 0 && pixel_x <= 160 {
                        // TODO: X flip
                        let bit = idx_x;
                        let mut pixel = if (hi >> bit) & 1 == 1 { 2 } else { 0 };
                        if (lo >> bit) & 1 == 1 { pixel |= 1 };
                        // TODO: palette number
                        let palette_number = 0;
                        let colors = self.sprite_palette(palette_number);
                        let color = colors[pixel];
                        // TODO: Check for existing pixels
                        if pixel != 0 {
                            self.set_pixel(pixel_x as u8, ly as u8, color)
                        }
                    }
                }
            }
            n += 4
        }
    }

    // FIXME: This may be super wasteful and constantly realloc
    fn sprite_palette(&self, palette: u8) -> [u8; 4] {
        [
            0,
            (palette >> 2) & 3,
            (palette >> 4) & 3,
            (palette >> 6) & 3,
        ]
    }

    fn set_mode(&mut self, mode: Mode) {
        let cleared = self.lcd & 0b1111_1100;

        match mode {
            HBlank      => self.lcd = cleared,
            VBlank      => self.lcd = cleared + 1,
            OamRead     => self.lcd = cleared + 2,
            LcdTransfer => self.lcd = cleared + 3
        }

        self.mode = mode;
    }

    pub fn load(&self, address: u16) -> u8 {
        if address < 0x8000 || address > 0x9FFF {
            panic!(); // TODO: Fix this
        }

        self.vram[address as usize & 0x1FFF]
    }

    pub fn store(&mut self, address: u16, value: u8) {
        if address < 0x8000 || address > 0x9FFF {
            panic!(); // TODO: Fix this
        }

        self.vram[address as usize & 0x1FFF] = value;
    }

    pub fn oam_load(&self, address: u16) -> u8 {
        let idx = (address / 4) as usize;
        let attr = address % 4;
        let sprite = self.oam[idx];
        match attr {
            0 => sprite.x,
            1 => sprite.y,
            2 => sprite.index,
            3 => sprite.attrs,
            _ => unreachable!(),
        }
    }

    pub fn oam_store(&mut self, address: u16, value: u8) {
        let idx = (address / 4) as usize;
        let attr = address % 4;
        let sprite = &mut self.oam[idx];
        match attr {
            0 => sprite.x = value,
            1 => sprite.y = value,
            2 => sprite.index = value,
            3 => sprite.attrs = value,
            _ => unreachable!(),
        }
    }

    fn set_pixel(&mut self, x: u8, y: u8, color: u8) {
        let offset = y as usize * 160 + x as usize;
        let color = color as usize;
        self.frame[offset] = COLOR_MAP[color].0;
        self.frame[offset+1] = COLOR_MAP[color].1;
        self.frame[offset+2] = COLOR_MAP[color].2;
    }

    fn clear_frame(&mut self) {
        for v in &mut self.frame {
            *v = 0;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    x: u8,
    y: u8,
    index: u8,
    attrs: u8
}

impl Sprite {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            index: 0,
            attrs: 0,
        }
    }
}
pub static COLOR_MAP: [(u8, u8, u8); 4] = [
    (0x9B, 0xBC, 0x0F),
    (0x8B, 0xAC, 0x0F),
    (0x30, 0x62, 0x30),
    (0x0F, 0x38, 0x0F),
];
