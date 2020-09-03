#![allow(dead_code)]

pub mod cartridge;
pub mod cpu;
pub mod gpu;
pub mod opcodes;
pub mod memory;
pub mod register;
pub mod timer;

use std::fs::File;
// use std::time::Duration;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;

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
    let cartridge = cartridge::Cartridge::load(&mut file);
    dbg!(cartridge.rom.len());

    let mut cpu = cpu::Cpu::new(cartridge);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 480, 432)
        .position_centered()
        .resizable()
        .allow_highdpi()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.clear();
    canvas.present();

    let tex_creator = canvas.texture_creator();
    let mut texture = tex_creator.create_texture_target(PixelFormatEnum::RGB24, 160, 144).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let prev_cy = cpu.cycles;
        cpu.step();
        let elapsed = cpu.cycles - prev_cy;

        let timer_int = cpu.memory.timer.tick(elapsed / 4);
        if timer_int {
            cpu.request_interrupt(4);
        }

        let lcd_on = cpu.memory.gpu.control & 0x80 > 0;
        cpu.memory.gpu.step(elapsed, lcd_on);

        if cpu.memory.gpu.new_frame && lcd_on {
            texture.update(None, &cpu.memory.gpu.frame, 160 * 3).unwrap();
            canvas.clear();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
            cpu.memory.gpu.new_frame = false;
            // ::std::thread::sleep(Duration::from_secs_f64( 1.0 / 60.0));
        }

        if cpu.memory.gpu.interrupts > 0 {
            cpu.request_interrupt(cpu.memory.gpu.interrupts);
            cpu.memory.gpu.interrupts = 0;
        }

        cpu.interrupt();

        if !cpu.memory.serial.is_empty() {
            let c = cpu.memory.serial.remove(0);
            print!("{}", c);
        }
    }
}
