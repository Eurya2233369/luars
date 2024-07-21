use super::instruction::Instruction;
use crate::api::{
    basic::{Arithmetic, BasicType, Comparison},
    lua_vm::LuaVM,
};

// R(A)-=R(A+2); pc+=sBx
pub fn for_prep(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, sbx) = i.a_sbx();
    a += 1;

    if vm.type_enum_id(a) == BasicType::LUA_TSTRING {
        vm.push_number(vm.to_number(a));
        vm.replace(a);
    }
    if vm.type_enum_id(a + 1) == BasicType::LUA_TSTRING {
        vm.push_number(vm.to_number(a + 1));
        vm.replace(a + 1);
    }
    if vm.type_enum_id(a + 2) == BasicType::LUA_TSTRING {
        vm.push_number(vm.to_number(a + 2));
        vm.replace(a + 2);
    }

    vm.push_value(a);
    vm.push_value(a + 2);
    vm.arith(Arithmetic::LUA_OPSUB);
    vm.replace(a);
    vm.add_pc(sbx);
}

// R(A)+=R(A+2);
// if R(A) <?= R(A+1) then {
//   pc+=sBx; R(A+3)=R(A)
// }
pub fn for_loop(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, sbx) = i.a_sbx();
    a += 1;

    // R(A)+=R(A+2);
    vm.push_value(a + 2);
    vm.push_value(a);
    vm.arith(Arithmetic::LUA_OPADD);
    vm.replace(a);

    let positive_step = vm.to_number(a + 2) >= 0.0;
    if positive_step && vm.compare(a, a + 1, Comparison::LUA_OPLE)
        || !positive_step && vm.compare(a + 1, a, Comparison::LUA_OPLE)
    {
        // pc+=sBx; R(A+3)=R(A)
        vm.add_pc(sbx);
        vm.copy(a, a + 3);
    }
}
