use super::instruction::Instruction;
use crate::api::lua_vm::LuaVM;

// R(A) := UpValue[B][RK(C)]
pub fn tab_up(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, _, c) = i.abc();
    a += 1;

    vm.push_global_table();
    vm.get_rk(c);
    vm.table(-2);
    vm.replace(a);
    vm.pop(1);
}
