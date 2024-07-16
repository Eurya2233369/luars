use super::instruction::Instruction;
use crate::api::{
    basic::{Arithmetic, Comparison},
    lua_vm::LuaVM,
};

pub fn for_prep(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, sbx) = i.a_sbx();
    a += 1;
    // R(A)-=R(A+2)
    vm.push_value(a);
    vm.push_value(a + 2);
    vm.arith(Arithmetic::LUA_OPSUB);
    vm.replace(a);
    //pc += sBx
    vm.add_pc(sbx);
}

pub fn for_loop(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, sbx) = i.a_sbx();
    a += 1;
    // R(A)+=R(A+2);
    vm.push_value(a + 2);
    vm.push_value(a);
    vm.arith(Arithmetic::LUA_OPADD);
    vm.replace(a);
    // R(A) = 0
    let positive_step = vm.to_number(a + 2).is_sign_positive();
    if positive_step && vm.compare(a, a + 1, Comparison::LUA_OPLE)
        || !positive_step && vm.compare(a + 1, a, Comparison::LUA_OPLE)
    {
        //R(A+3)=R(A)
        vm.add_pc(sbx);
        vm.copy(a, a + 3);
    }
}
