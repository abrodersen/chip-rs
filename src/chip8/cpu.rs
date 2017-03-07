
use std::io;
use std::cmp;

use super::op::{self, Opcode};
use super::timer::Timer;
use super::display::{self, Display, Point, Sprite};
use super::input::Input;

use rand::Rng;

const RAM_SIZE : usize = 4096;
const PC_START : usize = 0x200;
const SPRITE_BASE : usize = 0x010;
const SPRITE_ALIGN : usize = 0x10;

pub struct Cpu<D : Display, R : Rng, I : Input> {
    gp_reg: Box<[u8]>,
    i_reg:  u16,
    delay_timer: Timer,
    sound_timer: Timer,

    rng: R,
    display: D,
    input: I,

    pc_reg: u16,
    sp_reg: u8,
    stat_reg: bool,

    stack: Box<[u16]>,
    ram: Box<[u8]>,
}

//let ZERO: Sprite = { Sprite::new(&[0xF0, 0x90, 0x90, 0x90, 0xF0]) };
    // pub static ref ONE:   Sprite = { Sprite::new(&[0x20, 0x60, 0x20, 0x20, 0x70]) };
    // pub static ref TWO:   Sprite = { Sprite::new(&[0xF0, 0x10, 0xF0, 0x80, 0xF0]) };
    // pub static ref THREE: Sprite = { Sprite::new(&[0xF0, 0x10, 0xF0, 0x10, 0xF0]) };
    // pub static ref FOUR:  Sprite = { Sprite::new(&[0x90, 0x90, 0xF0, 0x10, 0x10]) };
    // pub static ref FIVE:  Sprite = { Sprite::new(&[0xF0, 0x80, 0xF0, 0x10, 0xF0]) };
    // pub static ref SIX:   Sprite = { Sprite::new(&[0xF0, 0x80, 0xF0, 0x90, 0xF0]) };
    // pub static ref SEVEN: Sprite = { Sprite::new(&[0xF0, 0x10, 0x20, 0x40, 0x40]) };
    // pub static ref EIGHT: Sprite = { Sprite::new(&[0xF0, 0x90, 0xF0, 0x90, 0xF0]) };
    // pub static ref NINE:  Sprite = { Sprite::new(&[0xF0, 0x90, 0xF0, 0x10, 0xF0]) };
    // pub static ref A:     Sprite = { Sprite::new(&[0xF0, 0x90, 0xF0, 0x90, 0x90]) };
    // pub static ref B:     Sprite = { Sprite::new(&[0xE0, 0x90, 0xE0, 0x90, 0xE0]) };
    // pub static ref C:     Sprite = { Sprite::new(&[0xF0, 0x80, 0x80, 0x80, 0xF0]) };
    // pub static ref D:     Sprite = { Sprite::new(&[0xE0, 0x90, 0x90, 0x90, 0xE0]) };
    // pub static ref E:     Sprite = { Sprite::new(&[0xF0, 0x80, 0xF0, 0x80, 0xF0]) };
    // pub static ref F:     Sprite = { Sprite::new(&[0xF0, 0x80, 0xF0, 0x80, 0x80]) };

impl<D, R, I> Cpu<D, R, I>  where D: Display, R : Rng, I : Input {
    pub fn new(rng: R, display: D, input: I) -> Self {
        let sprites = [
            Sprite::new(&[0xF0, 0x90, 0x90, 0x90, 0xF0]), // zero
            Sprite::new(&[0x20, 0x60, 0x20, 0x20, 0x70]), // one
            Sprite::new(&[0xF0, 0x10, 0xF0, 0x80, 0xF0]), // two
            Sprite::new(&[0xF0, 0x10, 0xF0, 0x10, 0xF0]), // three
            Sprite::new(&[0x90, 0x90, 0xF0, 0x10, 0x10]), // four
            Sprite::new(&[0xF0, 0x80, 0xF0, 0x10, 0xF0]), // five
            Sprite::new(&[0xF0, 0x80, 0xF0, 0x90, 0xF0]), // six
            Sprite::new(&[0xF0, 0x10, 0x20, 0x40, 0x40]), // seven
            Sprite::new(&[0xF0, 0x90, 0xF0, 0x90, 0xF0]), // eight
            Sprite::new(&[0xF0, 0x90, 0xF0, 0x10, 0xF0]), // nine
            Sprite::new(&[0xF0, 0x90, 0xF0, 0x90, 0x90]), // a
            Sprite::new(&[0xE0, 0x90, 0xE0, 0x90, 0xE0]), // b
            Sprite::new(&[0xF0, 0x80, 0x80, 0x80, 0xF0]), // c
            Sprite::new(&[0xE0, 0x90, 0x90, 0x90, 0xE0]), // d
            Sprite::new(&[0xF0, 0x80, 0xF0, 0x80, 0xF0]), // e
            Sprite::new(&[0xF0, 0x80, 0xF0, 0x80, 0x80]), // f
        ];

        let mut ram = vec![0; RAM_SIZE].into_boxed_slice();

        for (index, sprite) in sprites.iter().enumerate() {
            let start_address = SPRITE_BASE + index * SPRITE_ALIGN;
            let data = sprite.raw();
            let end_address = start_address + data.len();
            ram[start_address..end_address].copy_from_slice(data);
        }

        Cpu {
            gp_reg: vec![0; 16].into_boxed_slice(),
            i_reg: 0,
            delay_timer: Timer::new(),
            sound_timer: Timer::new(),
            rng: rng,
            display: display,
            input: input,
            pc_reg: 0,
            sp_reg: 0,
            stat_reg: false,
            stack: vec![0; 16].into_boxed_slice(),
            ram: ram,
        }
    }

    pub fn load_program(&mut self, src: &mut io::Read) -> Result<usize, io::Error>  {
        let mut buffer = Vec::new();
        try!(src.read_to_end(&mut buffer));
        let max = cmp::min(buffer.len(), self.ram.len() - PC_START);
        for i in 0..max {
            self.ram[i + PC_START] = buffer[i];
        }
        Ok(max)
    }

    fn get_word(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn get_dword(&self, addr: u16) -> u16 {
        let base = addr as usize;
        let upper = self.ram[base] as u16;
        let lower = self.ram[base + 1] as u16;
        (upper << 8) | lower
    }

    fn get_slice(&self, addr: u16, size: u16) -> &[u8] {
        let start = addr as usize;
        let end = (addr + size) as usize;
        self.ram[start..end].into()
    }

    fn set_word(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val
    }

    fn pop_stack(&mut self) -> u16 {
        let sp = self.sp_reg - 1;
        self.sp_reg = sp;
        self.stack[sp as usize]
    }

    fn push_stack(&mut self, val: u16) {
        self.stack[self.sp_reg as usize] = val;
        self.sp_reg += 1;
    }    

    fn get_reg(&self, reg: u8) -> u8 {
        self.gp_reg[reg as usize]
    }

    fn set_reg(&mut self, reg: u8, val: u8) {
        self.gp_reg[reg as usize] = val
    }

    fn set_i(&mut self, val: u16) {
        self.i_reg = val
    }

    pub fn start(&mut self) {
        self.pc_reg = PC_START as u16;

        loop {
            let op = self.get_dword(self.pc_reg);
            let instr = op::Instruction::new(op);
            println!("{:#x}: {:?}", self.pc_reg, instr.get_opcode());

            self.pc_reg += 2;

            self.run(instr);

            self.delay_timer.tick();
            self.sound_timer.tick();
        }
    }

    fn run(&mut self, instr: op::Instruction) {
        let op = instr.get_opcode();

        match op {
            // Clear the screen 
            Opcode::Cls => {
                self.display.clear();
            }
            // Return from a subroutine 
            Opcode::Ret => {
                let new_pc = self.pop_stack();
                self.pc_reg = new_pc;
            }
            // unused opcode
            Opcode::Sys => {
                println!("Ignoring {:?} opcode", op);
            },
            // Jump to address
            Opcode::Jp => {
                self.pc_reg = instr.get_addr();
            }
            // Call a subroutine by pushing the program counter on to the stack
            Opcode::Call => {
                let pc = self.pc_reg;
                self.push_stack(pc);
                self.pc_reg = instr.get_addr();
            }
            // skip if register equals immediate value
            Opcode::Sei => {
                if self.get_reg(instr.get_x()) == instr.get_byte() {
                    self.pc_reg += 2;
                }
            }
            // skip if register is not equal to immediate value
            Opcode::Snei => {
                if self.get_reg(instr.get_x()) != instr.get_byte() {
                    self.pc_reg += 2;
                }
            }
            // Load immeidate value into register
            Opcode::Ldi => {
                self.set_reg(instr.get_x(), instr.get_byte());
            }
            // 
            Opcode::Addi => {
                let value = self.get_reg(instr.get_x());
                let addition = value.wrapping_add(instr.get_byte());
                self.set_reg(instr.get_x(), addition);
            }
            // Load register y into register x
            Opcode::Ld => {
                let value = self.get_reg(instr.get_y());
                self.set_reg(instr.get_x(), value);
            }
            Opcode::And => {
                let left = self.get_reg(instr.get_x());
                let right = self.get_reg(instr.get_y());
                self.set_reg(instr.get_x(), left & right);
            }
            // add x and y registers and store in x
            Opcode::Add => {
                let left = self.get_reg(instr.get_x());
                let right = self.get_reg(instr.get_y());
                let (result, over) = left.overflowing_add(right);
                self.set_reg(instr.get_x(), result);
                self.stat_reg = over;
            }
            // subtract y from x and store in x
            Opcode::Sub => {
                let left = self.get_reg(instr.get_x());
                let right = self.get_reg(instr.get_y());
                let (result, over) = left.overflowing_sub(right);
                self.set_reg(instr.get_y(), result);
                self.stat_reg = over;
            }
            // load immediate into I register
            Opcode::Ldl => {
                self.set_i(instr.get_addr());
            }
            // generate a random number
            Opcode::Rnd => {
                let num: u8 = self.rng.gen();
                let masked = num & instr.get_byte();
                self.set_reg(instr.get_x(), masked);
            }
            // Draw a sprite to the screen
            Opcode::Drw => {
                let x = self.get_reg(instr.get_x());
                let y = self.get_reg(instr.get_y());
                let point = Point { x: x, y: y };
                
                // Do this to avoid borrow checker complaints
                let sprite = {
                    let data = self.get_slice(self.i_reg, instr.get_nibble() as u16);
                    Sprite::new(data)
                };

                self.display.draw(point, &sprite);
            }
            // Skip next instruction if key = register x is pressed
            Opcode::Skp => {
                let key = self.get_reg(instr.get_x());
                if self.input.is_pressed(key) {
                    self.pc_reg += 2;
                }
            }
            Opcode::Sknp => {
                let key = self.get_reg(instr.get_x());
                if !self.input.is_pressed(key) {
                    self.pc_reg += 2;
                }
            }
            // load delay timer into register x
            Opcode::Ldrt => {
                let time = self.delay_timer.get();
                self.set_reg(instr.get_x(), time);
            }
            Opcode::Ldk => {

            }
            // Load immediate into delay timer
            Opcode::Ldwt => {
                let time = self.get_reg(instr.get_x());
                self.delay_timer.set(time);
            }
            // Set I to sprite address x
            Opcode::Ldd => {
                let address = SPRITE_BASE as u16 + instr.get_x() as u16 * SPRITE_ALIGN as u16;
                self.set_i(address);
            }
            // Set sound timer to the value of register x
            Opcode::Ldst => {
                let time = self.get_reg(instr.get_x());
                self.sound_timer.set(time);
            }
            // Add register x to I register
            Opcode::Addl => {
                let added = self.i_reg + self.get_reg(instr.get_x()) as u16;
                self.set_i(added);
            }
            // Load binary coded decimal into I[0..2]
            Opcode::Ldb => {
                let base_addr = self.i_reg;
                let value = instr.get_x();
                for index in 0..2 {
                    let digit = value / (10 * (2 - index));
                    let addr = self.i_reg + (index as u16);
                    self.set_word(addr, digit);
                }
            }
            // Save registers into ram
            Opcode::Ldwr => {
                let base_addr = self.i_reg;
                for x in 0..instr.get_x() {
                    let offset = x as u16;
                    let val = self.get_reg(x);
                    self.set_word(base_addr + offset, val)
                }
            }
            // Load registers into ram
            Opcode::Ldrr => {
                let base_addr = self.i_reg;
                for x in 0..instr.get_x() {
                    let val = self.get_word(base_addr + (x as u16));
                    self.set_reg(x, val);
                }
            },
            
            _ => panic!("pc {:#x}: {:?} not implemented", self.pc_reg, op)
        }
    }
}
