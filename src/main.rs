#![allow(dead_code)]

pub mod cartridge;
pub mod console;
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
use sdl2::VideoSubsystem;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::video::WindowContext;
use sdl2::render::{Texture, WindowCanvas};

#[derive(Clap,Debug)]
#[clap(version = "0.1.0", author = "Xavier Perez <duosrx@gmail.com>")]
struct Opts {
    rom: PathBuf,
    #[clap(long, short)]
    trace: bool,
}
struct YeeboyWindow {
    pub canvas: WindowCanvas,
    texture: Texture,
    width: u32,
    height: u32,
    visible: bool,
}

impl YeeboyWindow {
    pub fn new(width: u32, height: u32, video: &VideoSubsystem) -> Self {
        let canvas = Self::make_canvas(width * 3, height * 3, video);
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_target(PixelFormatEnum::RGB24, width, height).unwrap();

        Self { canvas, texture, height, width, visible: true }
    }

    pub fn update(&mut self, frame: &[u8]) {
        if !self.visible {
            return
        }

        self.texture.update(None, &frame, self.width as usize * 3).unwrap();
        self.canvas.clear();
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present()
    }

    pub fn toggle(&mut self) {
        if self.visible {
            self.hide();
        } else {
            self.show();
        }

        self.visible = !self.visible;
    }

    pub fn hide(&mut self) { self.canvas.window_mut().hide() }
    pub fn show(&mut self) { self.canvas.window_mut().show() }

    fn make_canvas(width: u32, height: u32, video: &VideoSubsystem) -> WindowCanvas {
        let window = video.window("OAM Viewer", width, height)
            .resizable()
            .position(0, 0)
            .allow_highdpi()
            .opengl()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .build()
            .unwrap();

        canvas
    }
}

fn main() {
    let opts = Opts::parse();
    let mut file = File::open(opts.rom).unwrap();
    let cartridge = cartridge::Cartridge::load(&mut file);
    dbg!(&cartridge.headers);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut oam = YeeboyWindow::new(160, 144, &video_subsystem);
    let mut window = YeeboyWindow::new(160, 144, &video_subsystem);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut now = Instant::now();
    let mut console = console::Console::new(cartridge, opts.trace);

    'running: loop {
        console.step();

        if console.new_frame() {
            window.update(&console.cpu.memory.gpu.frame);
            oam.update(&console.cpu.memory.gpu.render_debug_sprites());

            // This should be outside of the new_frame condition but due to
            // a perf regression in SDL 2.0.9 we have to leave it here to
            // prevent horribly slow polling performance. Meh.
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(Keycode::O), .. } => {
                        oam.toggle();
                    }
                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        if let Some(button) = keycode_to_button(keycode) {
                            console.key_down(button);
                        }
                    }
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        if let Some(button) = keycode_to_button(keycode) {
                            console.key_up(button);
                        }
                    }
                    _ => {}
                }
            }

            // ::std::thread::sleep(Duration::from_secs_f64(1.0/200.0));

            if now.elapsed().as_secs() > 1 {
                window.canvas
                    .window_mut()
                    .set_title(&format!("YeeBoy - {} FPS", console.cpu.memory.gpu.frame_count))
                    .unwrap();
                console.cpu.memory.gpu.frame_count = 0;
                now = Instant::now();
            }
        }
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
