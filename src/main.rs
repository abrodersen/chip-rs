
#[macro_use]
extern crate lazy_static;
extern crate sdl2;

mod chip8;

use chip8::cpu;
use chip8::display;

use sdl2::video;

use std::env;
use std::fs;

const PIXEL_WIDTH : u32 = 64;
const PIXEL_HEIGHT : u32 = 32;
const SCALE_FACTOR : u32 = 30;

fn get_window(context: &sdl2::VideoSubsystem) -> video::Window {
    let width = PIXEL_WIDTH * SCALE_FACTOR;
    let height = PIXEL_HEIGHT * SCALE_FACTOR;
    context.window("CHIP-RS", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap()
}

fn main() {

    let filename = env::args().nth(1).unwrap();
    let mut file = fs::File::open(filename).unwrap();
    let size = file.metadata().unwrap().len();

    println!("file size: {} bytes", size);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = get_window(&video_subsystem);

    let mut display = display::sdl::SdlDisplay::new(SCALE_FACTOR, window);
    let mut cpu = cpu::Cpu::new(display);

    let count = cpu.load_program(&mut file).unwrap();
    println!("loaded {} bytes into ram", count);

    cpu.start();

}
