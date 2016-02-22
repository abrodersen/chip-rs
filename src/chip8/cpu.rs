use std::io;
use std::cmp;

use super::op::{self, Opcode};

const RAM_SIZE : usize = 4096;
const PC_START : usize = 0x200;

pub struct Cpu {
    gp_reg: Box<[u8]>,
    i_reg:  u16,
    //timer reg
    //sound reg

    pc_reg: u16,
    sp_reg: u8,

    stack: Box<[u16]>,
    ram: Box<[u8]>,
}


impl Cpu {
    pub fn new() -> Self {
        Cpu {
            gp_reg: vec![0; 16].into_boxed_slice(),
            i_reg: 0,
            pc_reg: 0,
            sp_reg: 0,
            stack: vec![0; 16].into_boxed_slice(),
            ram: vec![0; RAM_SIZE].into_boxed_slice(),
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

    fn set_word(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val
    }

    fn get_dword(&self, addr: u16) -> u16 {
        let base = addr as usize;
        let upper = self.ram[base] as u16;
        let lower = self.ram[base + 1] as u16;
        (upper << 8) | lower
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

            self.run(instr);

            self.pc_reg = self.pc_reg + 2;
        }
    }

    fn run(&mut self, instr: op::Instruction) {
        let op = instr.get_opcode();
        match op {
            Opcode::Jp => {
                self.pc_reg = instr.get_addr() - 2;
            },
            Opcode::Ldi => {
                self.set_reg(instr.get_x(), instr.get_byte());
            },
            Opcode::Ldl => {
                self.set_i(instr.get_addr());
            },
            Opcode::Ldwr => {
                let base_addr = self.i_reg;
                for x in 0..instr.get_x() {
                    let offset = x as u16;
                    let val = self.get_reg(x);
                    self.set_word(base_addr + offset, val)
                }
            }
            _ => panic!("pc {:#x}: {:?} not implemented", self.pc_reg, op)
        }
    }
}
