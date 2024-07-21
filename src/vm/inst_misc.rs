use super::instruction::Instruction;
use crate::api::lua_vm::LuaVM;

// R(A) := R(B)
pub fn misc_move(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, mut b, _) = i.abc();
    a += 1;
    b += 1;

    vm.copy(b, a);
}

// pc+=sBx; if (A) close all upvalues >= R(A - 1)
pub fn misc_jump(i: u32, vm: &mut dyn LuaVM) {
    let (a, sbx) = i.a_sbx();
    vm.add_pc(sbx);

    if a != 0 {
        todo!();
    }
}
