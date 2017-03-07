
#[macro_use]
extern crate lazy_static;
extern crate sdl2;
extern crate rand;
extern crate chan_signal;

mod chip8;

use chip8::cpu;
use chip8::display;
use chip8::input;

use rand::{Rng, SeedableRng};

use sdl2::video;

use chan_signal::Signal;

use std::env;
use std::fs;
use std::thread;

fn main() {

    let filename = env::args().nth(1).unwrap();
    let mut file = fs::File::open(filename).unwrap();
    let size = file.metadata().unwrap().len();

    println!("file size: {} bytes", size);

    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);

    thread::spawn(move || {
        let seed = &[0xdeadbeef];
        let mut rng = rand::StdRng::from_seed(seed);

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let mut input_subsystem = sdl_context.event_pump().unwrap();

        let mut display = display::sdl::SdlDisplay::new(&video_subsystem);
        let mut input = input::sdl::SdlInput::new(&mut input_subsystem);
        //let mut display = display::debug::DebugDisplay::new();
        let mut cpu = cpu::Cpu::new(rng, display, input);

        let count = cpu.load_program(&mut file).unwrap();
        println!("loaded {} bytes into ram", count);

        cpu.start();
    });
    
    signal.recv().unwrap();

}
