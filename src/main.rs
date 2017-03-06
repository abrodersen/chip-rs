
#[macro_use]
extern crate lazy_static;

mod chip8;

use chip8::cpu;
use chip8::display;

use std::env;
use std::fs;

fn main() {

    let filename = env::args().nth(1).unwrap();
    let mut file = fs::File::open(filename).unwrap();
    let size = file.metadata().unwrap().len();

    println!("file size: {} bytes", size);

    let mut display = display::debug::DebugDisplay::new();
    let mut cpu = cpu::Cpu::new(display);

    let count = cpu.load_program(&mut file).unwrap();
    println!("loaded {} bytes into ram", count);

    cpu.start();

}
