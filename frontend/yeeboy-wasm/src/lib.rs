mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// #[wasm_bindgen]
// pub fn drmario() -> *const u8 {
//     let bytes = include_bytes!("../../../roms/drmario.gb");
//     bytes.as_ptr()
// }

#[wasm_bindgen]
pub struct Console {
    console: yeeboy::console::Console,
}

#[wasm_bindgen]
impl Console {
    pub fn new() -> Console {
        let bytes = include_bytes!("../../../roms/drmario.gb");
        let cartridge = yeeboy::cartridge::Cartridge::load(bytes.to_vec());
        let console = yeeboy::console::Console::new(cartridge, false);
        Console { console }
    }

    pub fn step(&mut self) {
        self.console.cpu.memory.gpu.new_frame = false;
        self.console.step();
    }

    pub fn end_frame(&mut self) {
        self.console.cpu.memory.gpu.new_frame = false;
    }

    pub fn new_frame(&self) -> bool {
        self.console.new_frame()
    }

    pub fn frame(&self) -> *const u8 {
        self.console.frame().clone().as_ptr()
    }

    pub fn pc(&self) -> u16 {
        self.console.cpu.pc
    }

    pub fn regs(&self) -> String {
        use yeeboy::register::Register16::*;
        format!(
            "AF:{:04X} BC:{:04X} DE:{:04X} HL:{:04X} SP:{:04X} [{}] {:04X}",
            self.console.cpu.registers.get16(AF),
            self.console.cpu.registers.get16(BC),
            self.console.cpu.registers.get16(DE),
            self.console.cpu.registers.get16(HL),
            self.console.cpu.registers.get16(SP),
            "",
            self.console.cpu.pc,
        )
    }
}
