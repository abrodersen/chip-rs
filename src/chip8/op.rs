#[derive(Debug)]
pub enum Opcode {
    Sys,
    Cls,
    Ret,
    Jp,
    Call,
    Sei,
    Snei,
    Se,
    Ldi,
    Addi,
    Ld,
    Or,
    And,
    Xor,
    Add,
    Sub,
    Shr,
    Subn,
    Shl,
    Sne,
    Ldl,
    Jpa,
    Rnd,
    Drw,
    Skp,
    Sknp,
    Ldrt,
    Ldk,
    Ldwt,
    Ldst,
    Addl,
    Ldd,
    Ldb,
    Ldwr,
    Ldrr,
}

pub struct Instruction(u16);

macro_rules! op {
    ($x:pat) => (($x, _, _, _));
    ($x:pat, $y:pat) => (($x, $y, _, _));
    ($x:pat, $y:pat, $z:pat) => (($x, $y, $z, _));
    ($x:pat, $y:pat, $z:pat, $w:pat) => (($x, $y, $z, $w));
}

impl Instruction {
    pub fn new (inst: u16) -> Instruction {
        Instruction(inst)
    }

    pub fn get_opcode (&self) -> Opcode {
        let a = self.0 >> 12;
        let b = (self.0 >> 8) & 0xf;
        let c = (self.0 >> 4) & 0xf;
        let d = self.0 & 0xf;

        match (a, b, c, d) {
            op!(0, 0, 0xE, 0) => Opcode::Cls,
            op!(0, 0, 0xE, 0xE) => Opcode::Ret,
            op!(0) => Opcode::Sys,
            op!(1) => Opcode::Jp,
            op!(2) => Opcode::Call,
            op!(3) => Opcode::Sei,
            op!(4) => Opcode::Snei,
            op!(5, _, _, 0) => Opcode::Se,
            op!(6) => Opcode::Ldi,
            op!(7) => Opcode::Addi,
            op!(8, _, _, 0) => Opcode::Ld,
            op!(8, _, _, 1) => Opcode::Or,
            op!(8, _, _, 2) => Opcode::And,
            op!(8, _, _, 3) => Opcode::Xor,
            op!(8, _, _, 4) => Opcode::Add,
            op!(8, _, _, 5) => Opcode::Sub,
            op!(8, _, _, 6) => Opcode::Shr,
            op!(8, _, _, 7) => Opcode::Subn,
            op!(8, _, _, 0xE) => Opcode::Shl,
            op!(9, _, _, 0) => Opcode::Sne,
            op!(0xA) => Opcode::Ldl,
            op!(0xB) => Opcode::Jpa,
            op!(0xC) => Opcode::Rnd,
            op!(0xD) => Opcode::Drw,
            op!(0xE, _, 0x9, 0xE) => Opcode::Skp,
            op!(0xE, _, 0xA, 0x1) => Opcode::Sknp,
            op!(0xF, _, 0x0, 0x7) => Opcode::Ldrt,
            op!(0xF, _, 0x0, 0xA) => Opcode::Ldk,
            op!(0xF, _, 0x1, 0x5) => Opcode::Ldwt,
            op!(0xF, _, 0x1, 0x8) => Opcode::Ldst,
            op!(0xF, _, 0x1, 0xE) => Opcode::Addl,
            op!(0xF, _, 0x2, 0x9) => Opcode::Ldd,
            op!(0xF, _, 0x3, 0x3) => Opcode::Ldb,
            op!(0xF, _, 0x5, 0x5) => Opcode::Ldwr,
            op!(0xF, _, 0x6, 0x5) => Opcode::Ldrr,
            _ => panic!("unrecognized opcode"),
        }
    }

    pub fn get_addr(&self) -> u16 {
        (self.0 & 0xFFF)
    }

    pub fn get_byte(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    pub fn get_nibble(&self) -> u8 {
        (self.0 & 0xF) as u8
    }

    pub fn get_x(&self) -> u8 {
        ((self.0 >> 8) & 0xF) as u8
    }

    pub fn get_y(&self) -> u8 {
        ((self.0 >> 4) & 0xF) as u8
    }
}

#[test]
fn get_byte() {
    
}
