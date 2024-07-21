use crate::api::lua_vm::LuaVM;

use super::{
    inst_call::{call, call_return, closure, tail_call, vararg}, inst_for::{for_loop, for_prep}, inst_load::{load_bool, load_k, load_kx, load_nil}, inst_misc::{misc_jump, misc_move}, inst_operators::{
        binary_add, binary_band, binary_bnot, binary_bor, binary_bxor, binary_div, binary_idiv,
        binary_mod, binary_mul, binary_pow, binary_shl, binary_shr, binary_sub, binary_unm, concat,
        eq, le, len, lt, not, test, test_set,
    }, inst_table::{table, new_table, set_list, set_table}, inst_upvalue::tab_up, opcode::{
        code_map, OP_ADD, OP_BAND, OP_BNOT, OP_BOR, OP_BXOR, OP_CALL, OP_CLOSURE, OP_CONCAT, OP_DIV, OP_EQ, OP_FORLOOP, OP_FORPREP, OP_GETTABLE, OP_GETTABUP, OP_IDIV, OP_JMP, OP_LE, OP_LEN, OP_LOADBOOL, OP_LOADK, OP_LOADKX, OP_LOADNIL, OP_LT, OP_MOD, OP_MOVE, OP_MUL, OP_NEWTABLE, OP_NOT, OP_POW, OP_RETURN, OP_SELF, OP_SETLIST, OP_SETTABLE, OP_SHL, OP_SHR, OP_SUB, OP_TAILCALL, OP_TEST, OP_TESTSET, OP_UNM, OP_VARARG
    }
};

const MAXARG_BX: isize = (1 << 18) - 1; // 2^18 - 1
const MAXARG_S_BX: isize = MAXARG_BX >> 1; // Floor(2^18 - 1)/2)

pub trait Instruction {
    fn opname(self) -> &'static str;
    fn b_mode(self) -> u8;
    fn c_mode(self) -> u8;
    fn opmode(self) -> u8;
    fn opcode(self) -> u8;
    fn abc(self) -> (isize, isize, isize);
    fn a_bx(self) -> (isize, isize);
    fn a_sbx(self) -> (isize, isize);
    fn ax(self) -> isize;
    fn execute(self, vm: &mut dyn LuaVM);
}

impl Instruction for u32 {
    fn opname(self) -> &'static str {
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

    fn opcode(self) -> u8 {
        (self & 0x3F) as u8
    }

    fn abc(self) -> (isize, isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let c = (self >> 14 & 0x1FF) as isize;
        let b = (self >> 23 & 0x1FF) as isize;
        (a, b, c)
    }

    fn a_bx(self) -> (isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let bx = (self >> 14) as isize;
        (a, bx)
    }

    fn a_sbx(self) -> (isize, isize) {
        let (a, bx) = self.a_bx();
        (a, bx - MAXARG_S_BX)
    }

    fn ax(self) -> isize {
        (self >> 6) as isize
    }

    fn execute(self, vm: &mut dyn LuaVM) {
        match self.opcode() {
            OP_MOVE => misc_move(self, vm),
            OP_LOADK => load_k(self, vm),
            OP_LOADKX => load_kx(self, vm),
            OP_LOADBOOL => load_bool(self, vm),
            OP_LOADNIL => load_nil(self, vm),
            // TODO
            OP_GETTABUP => tab_up(self, vm),
            OP_GETTABLE => table(self, vm),
            OP_SETTABLE => set_table(self, vm),
            OP_NEWTABLE => new_table(self, vm),
            OP_SELF => call_return(self, vm),
            OP_ADD => binary_add(self, vm),
            OP_SUB => binary_sub(self, vm),
            OP_MUL => binary_mul(self, vm),
            OP_MOD => binary_mod(self, vm),
            OP_POW => binary_pow(self, vm),
            OP_DIV => binary_div(self, vm),
            OP_IDIV => binary_idiv(self, vm),
            OP_BAND => binary_band(self, vm),
            OP_BOR => binary_bor(self, vm),
            OP_BXOR => binary_bxor(self, vm),
            OP_SHL => binary_shl(self, vm),
            OP_SHR => binary_shr(self, vm),
            OP_UNM => binary_unm(self, vm),
            OP_BNOT => binary_bnot(self, vm),
            OP_NOT => not(self, vm),
            OP_LEN => len(self, vm),
            OP_CONCAT => concat(self, vm),
            OP_JMP => misc_jump(self, vm),
            OP_EQ => eq(self, vm),
            OP_LT => lt(self, vm),
            OP_LE => le(self, vm),
            OP_TEST => test(self, vm),
            OP_TESTSET => test_set(self, vm),
            OP_CALL => call(self, vm),
            OP_TAILCALL => tail_call(self, vm),
            OP_RETURN => call_return(self, vm),
            OP_FORLOOP => for_loop(self, vm),
            OP_FORPREP => for_prep(self, vm),
            // TODO
            OP_SETLIST => set_list(self, vm),
            OP_CLOSURE => closure(self, vm),
            OP_VARARG => vararg(self, vm),
            _ => {
                dbg!(self.opname());
                unimplemented!()
            }
        }
    }
}
