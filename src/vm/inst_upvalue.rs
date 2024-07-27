use super::instruction::Instruction;
use crate::api::lua_vm::{lua_upvalue_index, LuaAPI, LuaVM};

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

// R(A) := UpValue[B]
pub fn get_upval(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, mut b, _) = i.abc();
    a += 1;
    b += 1;
    vm.copy(lua_upvalue_index(b), a);
}

// UpValue[B] := R(A)
pub fn set_upval(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, mut b, _) = i.abc();
    a += 1;
    b += 1;
    vm.copy(a, lua_upvalue_index(b));
}

// R(A) := UpValue[B][RK(C)]
pub fn get_tabup(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, mut b, c) = i.abc();
    a += 1;
    b += 1;
    vm.get_rk(c);
    vm.table(lua_upvalue_index(b));
    vm.replace(a);
}

// UpValue[A][RK(B)] := RK(C)
pub fn set_tabup(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, b, c) = i.abc();
    a += 1;

    vm.get_rk(b);
    vm.get_rk(c);
    vm.set_table(lua_upvalue_index(a));
}
