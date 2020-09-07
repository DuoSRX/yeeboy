// use crate::memory::Memory;

pub static COLOR_MAP: [(u8, u8, u8); 4] = [
    (0x9B, 0xBC, 0x0F),
    (0x8B, 0xAC, 0x0F),
    (0x30, 0x62, 0x30),
    (0x0F, 0x38, 0x0F),
];

#[derive(Debug)]
enum Mode {
    VBlank,
    HBlank,
    OamRead,
    LcdTransfer,
}

use Mode::*;

pub struct Gpu {
    mode: Mode,
    pub cycles: u64,
    pub ly: u8,
    pub lyc: u8,
    pub lcd: u8,
    pub frame: Vec<u8>,
    pub frame_count: u64,
    pub vram: Vec<u8>,
    pub interrupts: u8,
    pub oam: Vec<Sprite>,
    pub new_frame: bool,
    pub control: u8,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub window_x: u8,
    pub window_y: u8,
    pub bg_palette: u8,
    pub obj_palette_0: u8,
    pub obj_palette_1: u8,
}

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            mode: HBlank,
            lcd: 0x80,
            cycles: 0,
            frame_count: 0,
            ly: 0,
            lyc: 0,
            scroll_x: 0,
            scroll_y: 0,
            window_x: 0,
            window_y: 0,
            control: 0,
            interrupts: 0,
            bg_palette: 0,
            obj_palette_0: 0,
            obj_palette_1: 0,
            frame: vec![0; 160 * 144 * 3],
            vram: vec![0; 0x2000],
            oam: vec![Sprite::new(); 0x40],
            new_frame: false,
        }
    }

    pub fn step(&mut self, cycles: u64) {
        self.cycles += cycles;
        self.interrupts = 0;

        match self.mode {
            OamRead if self.cycles >= 80 => {
                self.cycles -= 80;
                self.set_mode(LcdTransfer)
            }
            LcdTransfer if self.cycles >= 172 => {
                self.cycles -= 172;
                if self.lcd_on() {
                    if self.bg_priority() {
                        self.render_background();
                        if self.window_enabled() {
                            self.render_window();
                        }
                    } else {
                        self.clear_frame();
                    }
                    self.render_sprites();
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

        if self.ly == self.lyc {
            self.lcd |= 0x40;
            self.interrupts |= 2;
        }
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
        let tile_data = self.tile_data();
        let tile_map = self.tile_map();
        let y: u16 = ((scroll_y + ly) / 8) % 32;
        let y_offset = (scroll_y + ly) % 8;

        for px in 0..160 {
            let x = ((scroll_x + px) / 8) % 32;
            let tile = self.load(tile_map.wrapping_add(y * 32).wrapping_add(x));
            let ptr = match tile_data {
                0x9000 => (tile_data as i32 + (tile as i8 as i32 * 0x10)) as u16,
                _      => tile_data.wrapping_add(tile as u16 * 0x10),
            };
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

    fn render_window(&mut self) {
        let palette = self.bg_palette;
        let colors = [
            palette & 3,
            (palette >> 2) & 3,
            (palette >> 4) & 3,
            (palette >> 6) & 3,
        ];

        let ly = self.ly as u16;
        let tile_data = self.tile_data();
        let tile_map = self.window_tile_map();
        let window_y = ly - self.window_y as u16;
        let y = window_y / 8;
        let y_offset = window_y % 8;
        let window_x = self.window_x.wrapping_sub(7);

        for px in 0..=159 {
            if px < window_x {
                continue;
            }

            let x = (px - window_x) / 8;
            let tile = self.load(tile_map.wrapping_add(y * 32).wrapping_add(x as u16));
            let ptr = match tile_data {
                0x9000 => (tile_data as i32 + (tile as i8 as i32 * 0x10)) as u16,
                _      => tile_data.wrapping_add(tile as u16 * 0x10),
            };
            let ptr = ptr.wrapping_add(y_offset * 2);
            let p0 = self.load(ptr);
            let p1 = self.load(ptr + 1);
            let colb = 7 - px % 8;
            let pix0 = if p0 >> colb & 1 == 1 { 1 } else { 0 };
            let pix1 = if p1 >> colb & 1 == 1 { 2 } else { 0 };
            let coln = pix0 | pix1;
            let color = colors[coln as usize];
            self.set_pixel(px as u8, ly as u8, color);
        }
    }

    fn render_sprites(&mut self) {
        let sprite_height = if self.control & 4 > 0 { 16 } else { 8 };
        let ly = self.ly as i16;

        let mut n = 0;
        while n < 156 {
            let sprite = self.oam[n / 4];

            let y = sprite.y as i16 - 16;
            let x = sprite.x as i16 - 8;
            let on_scanline = (y <= ly) && (y + sprite_height) > ly;

            if on_scanline {
                let y_offset = if sprite.y_flip() {
                    (sprite_height - 1) - (ly - y)
                } else {
                    ly - y
                };
                let ptr = ((sprite.index as i32 * 16) + ((y_offset as i32) * 2)) as u16;
                let lo = self.load(0x8000 + ptr);
                let hi = self.load(0x8000 + ptr + 1);

                for idx_x in 0..=7 {
                    let pixel_x = x + idx_x;
                    if pixel_x >= 0 && pixel_x <= 160 {
                        let mut bit = idx_x;
                        if !sprite.x_flip() { bit = 7 - bit };
                        let mut pixel = if (hi >> bit) & 1 == 1 { 2 } else { 0 };
                        if (lo >> bit) & 1 == 1 { pixel |= 1 };
                        let palette= if sprite.attrs & 0x8 == 0 { self.obj_palette_0 } else { self.obj_palette_1 };
                        let color = self.sprite_pixel_color(palette, pixel);
                        if pixel != 0 && sprite.attrs & 0x80 == 0 || self.is_pixel_blank(pixel_x as u8, ly as u8) {
                            self.set_pixel(pixel_x as u8, ly as u8, color)
                        }
                    }
                }
            }
            n += 4
        }
    }

    fn sprite_pixel_color(&self, palette: u8, pixel: u8) -> u8 {
        match pixel {
            1 => (palette >> 2) & 3,
            2 => (palette >> 4) & 3,
            3 => (palette >> 6) & 3,
            _ => 0,
        }
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
            0 => sprite.y,
            1 => sprite.x,
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
            0 => sprite.y = value,
            1 => sprite.x = value,
            2 => sprite.index = value,
            3 => sprite.attrs = value,
            _ => unreachable!(),
        }
        // dbg!(sprite);
    }

    fn set_pixel(&mut self, x: u8, y: u8, color: u8) {
        let offset = y as usize * 160 + x as usize;
        let color = color as usize;
        self.frame[offset*3] = COLOR_MAP[color].0;
        self.frame[offset*3+1] = COLOR_MAP[color].1;
        self.frame[offset*3+2] = COLOR_MAP[color].2;
    }

    fn is_pixel_blank(&mut self, x: u8, y: u8) -> bool {
        let offset = y as usize * 160 + x as usize;
        self.frame[offset*3] == 0
            && self.frame[offset*3+1] == 0
            && self.frame[offset*3+2] == 0
    }

    fn clear_frame(&mut self) {
        for v in &mut self.frame {
            *v = 0;
        }
    }

    fn tile_data(&self) -> u16 {
        match self.control & 0x10 > 0 {
            true => 0x8000,
            _    => 0x9000,
        }
    }

    fn tile_map(&self) -> u16 {
        match self.control & 0x08 > 0 {
            true => 0x9C00,
            _    => 0x9800,
        }
    }

    fn window_tile_map(&self) -> u16 {
        match self.control & 0x40 > 0 {
            true => 0x9C00,
            _    => 0x9800,
        }
    }

    fn window_enabled(&self) -> bool {
        self.control & 0x20 != 0
    }

    fn bg_priority(&self) -> bool {
        self.control & 0x01 != 0
    }

    pub fn lcd_on(&self) -> bool {
        self.control & 0x80 != 0
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

    pub fn x_flip(&self) -> bool {
        self.attrs & 0x20 != 0
    }

    pub fn y_flip(&self) -> bool {
        self.attrs & 0x40 != 0
    }
}
