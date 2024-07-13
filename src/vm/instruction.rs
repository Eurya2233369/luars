const MAXINDEXRK: isize = 
pub trait Instruction {
    fn opcode(&self) -> u8;
    fn abc(&self) -> (isize, isize, isize);
    fn a_bx(&self) -> (isize, isize);
    fn a_sbx(&self) -> (isize, isize);
}

impl Instruction for u32 {
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
        (a, bx - MAXARG_SBX)
    }
}
