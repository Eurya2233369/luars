use super::opcode::code_map;

const MAXARG_BX: isize = (1 << 18) - 1; // 2^18 - 1
const MAXARG_S_BX: isize = MAXARG_BX >> 1; // Floor(2^18 - 1)/2)

pub trait Instruction {
    fn opname(&self) -> &'static str;
    fn b_mode(self) -> u8;
    fn c_mode(self) -> u8;
    fn opmode(self) -> u8;
    fn opcode(&self) -> u8;
    fn abc(&self) -> (isize, isize, isize);
    fn a_bx(&self) -> (isize, isize);
    fn a_sbx(&self) -> (isize, isize);
    fn ax(&self) -> isize;
}

impl Instruction for u32 {
    fn opname(&self) -> &'static str {
        code_map(self.opcode()).name
    }

    fn b_mode(self) -> u8 {
        code_map(self.opcode()).bmode
    }

    fn c_mode(self) -> u8 {
        code_map(self.opcode()).cmode
    }

    fn opmode(self) -> u8 {
        code_map(self.opcode()).opmode
    }

    fn opcode(&self) -> u8 {
        (self & 0x3F) as u8
    }

    fn abc(&self) -> (isize, isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let c = (self >> 14 & 0x1FF) as isize;
        let b = (self >> 23 & 0x1FF) as isize;
        (a, b, c)
    }

    fn a_bx(&self) -> (isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let bx = (self >> 14) as isize;
        (a, bx)
    }

    fn a_sbx(&self) -> (isize, isize) {
        let (a, bx) = self.a_bx();
        (a, bx - MAXARG_S_BX)
    }

    fn ax(&self) -> isize {
        (self >> 6) as isize
    }
}
