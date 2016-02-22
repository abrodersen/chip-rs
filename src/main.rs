mod chip8;

use chip8::cpu;

use std::env;
use std::fs;

fn main() {

    let filename = env::args().nth(1).unwrap();
    let mut file = fs::File::open(filename).unwrap();
    let size = file.metadata().unwrap().len();

    println!("file size: {} bytes", size);

    let mut cpu = cpu::Cpu::new();

    let count = cpu.load_program(&mut file).unwrap();
    println!("loaded {} bytes into ram", count);

    cpu.start();

}
