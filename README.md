# CHIP-RS

A CHIP-8 emulator written in [Rust](http://rust-lang.org). Rust is a systems language that provides numerous safety guarantees.

__Note:__ This emulator is only partially complete. It contains bugs and unimplemented opcodes. It is a Work in Progress™.

# Requirements

* Git
* Rust
* Cargo
* SDL2

Rust and Cargo are best installed using [Rustup](http://rustup.rs). Get SDL2 through your system package manager, or install from [source](https://www.libsdl.org/download-2.0.php).

# Installation

````
$ git clone https://github.com/abrodersen/chip-rs.git
$ cd chip-rs
$ cargo install
````

# Usage

Ensure that `$HOME/.cargo/bin` is on your path. To run the emulator:

````
$ chip-rs /path/to/rom.bin
````

A graphical window representing the emulator output will pop up. Use the 4x4 keypad from 1-4 to Z-V to enter input. The emulator currently does not produce any sound.

# License

The emulator code is distributed under the MIT license.
