#![allow(dead_code)]

pub mod cartridge;
pub mod cpu;
pub mod gpu;
pub mod input;
pub mod opcodes;
pub mod memory;
pub mod register;
pub mod timer;

use std::fs::File;
use std::path::PathBuf;
use std::time::{Instant, Duration};

use clap::Clap;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(Clap,Debug)]
#[clap(version = "0.1.0", author = "Xavier Perez <duosrx@gmail.com>")]
struct Opts {
    rom: PathBuf,
    #[clap(long, short)]
    trace: bool,
}

fn main() {
    let opts = Opts::parse();
    let mut file = File::open(opts.rom).unwrap();
    let cartridge = cartridge::Cartridge::load(&mut file);
    dbg!(&cartridge.headers);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("YeeBoy", 480, 432)
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

    let mut now = Instant::now();

    let mut cpu = cpu::Cpu::new(cartridge, opts.trace);

    'running: loop {
        let prev_cy = cpu.cycles;
        cpu.step();
        let elapsed = cpu.cycles - prev_cy;

        cpu.memory.gpu.step(elapsed);
        let lcd_on = cpu.memory.gpu.lcd_on();

        if cpu.memory.gpu.new_frame && lcd_on {
            texture.update(None, &cpu.memory.gpu.frame, 160 * 3).unwrap();
            canvas.clear();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
            cpu.memory.gpu.new_frame = false;
            cpu.memory.gpu.frame_count += 1;

            // This should be outside of the new_frame condition but due to
            // a perf regression in SDL 2.0.9 we have to leave it here to
            // prevent horribly slow polling performance. Meh.
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        if let Some(button) = keycode_to_button(keycode) {
                            cpu.memory.input.key_down(button);
                        }
                    }
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        if let Some(button) = keycode_to_button(keycode) {
                            cpu.memory.input.key_up(button);
                        }
                    }
                    _ => {}
                }
            }

            ::std::thread::sleep(Duration::from_secs_f64(1.0/70.0));

            if now.elapsed().as_secs() > 1 {
                canvas
                    .window_mut()
                    .set_title(&format!("YeeBoy - {} FPS", cpu.memory.gpu.frame_count))
                    .unwrap();
                cpu.memory.gpu.frame_count = 0;
                now = Instant::now();
            }
        }

        if cpu.memory.timer.tick(elapsed / 4) {
            cpu.request_interrupt(4);
        }

        if cpu.memory.gpu.interrupts > 0 {
            cpu.request_interrupt(cpu.memory.gpu.interrupts);
            cpu.memory.gpu.interrupts = 0;
        }

        cpu.interrupt();

        // if !cpu.memory.serial.is_empty() {
        //     let c = cpu.memory.serial.remove(0);
        //     print!("{}", c);
        // }
    }
}

fn keycode_to_button(keycode: Keycode) -> Option<input::Button> {
    match keycode {
        Keycode::LShift => Some(input::Button::Select),
        Keycode::Space => Some(input::Button::Start),
        Keycode::Left => Some(input::Button::Left),
        Keycode::Right => Some(input::Button::Right),
        Keycode::Down => Some(input::Button::Down),
        Keycode::Up => Some(input::Button::Up),
        Keycode::Z => Some(input::Button::B),
        Keycode::X => Some(input::Button::A),
        _ => None
    }
}
