use super::instruction::Instruction;
use crate::api::{
    basic::{Arithmetic, Comparison},
    lua_vm::LuaVM,
};

// R(A) := RK(B) op RK(C)
fn binary_arith(i: u32, vm: &mut dyn LuaVM, op: Arithmetic) {
    let (mut a, b, c) = i.abc();
    a += 1;
    vm.get_rk(b);
    vm.get_rk(c);
    vm.arith(op);
    vm.replace(a);
}

// R(A) := op R(B)
fn unary_arith(i: u32, vm: &mut dyn LuaVM, op: Arithmetic) {
    let (mut a, mut b, _) = i.abc();
    a += 1;
    b += 1;
    vm.push_value(b);
    vm.arith(op);
    vm.replace(a);
}

// +
pub fn binary_add(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPADD);
}

// -
pub fn binary_sub(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPSUB);
}

// *
pub fn binary_mul(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPMUL);
}

// %
pub fn binary_mod(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPMOD);
}

// ^
pub fn binary_pow(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPPOW);
}

// /
pub fn binary_div(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPDIV);
}

// //
pub fn binary_idiv(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPIDIV);
}

// &
pub fn binary_band(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPBAND);
}

// |
pub fn binary_bor(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPBOR);
}

// ~
pub fn binary_bxor(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPBXOR);
}

// <<
pub fn binary_shl(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPSHL);
}

// >>
pub fn binary_shr(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPSHR);
}

// -
pub fn binary_unm(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPUNM);
}

// ~
pub fn binary_bnot(i: u32, vm: &mut dyn LuaVM) {
    binary_arith(i, vm, Arithmetic::LUA_OPBNOT);
}

// R(A) := length of R(B)
pub fn len(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, mut b, _) = i.abc();
    a += 1;
    b += 1;

    vm.len(b);
    vm.replace(a);
}

// R(A) := R(B).. ... ..R(C)
pub fn concat(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, mut b, mut c) = i.abc();
    a += 1;
    b += 1;
    c += 1;

    let n = c - b + 1;
    vm.check_stack(n as usize);
    for i in b..(c + 1) {
        vm.push_value(i);
    }
    vm.concat(n);
    vm.replace(a);
}

// if ((RK(B) op RK(C)) ~= A) then pc++
fn compare(i: u32, vm: &mut dyn LuaVM, op: Comparison) {
    let (a, b, c) = i.abc();

    vm.get_rk(b);
    vm.get_rk(c);
    if vm.compare(-2, -1, op) != (a != 0) {
        vm.add_pc(1);
    }
    vm.pop(2);
}

// ==
pub fn eq(i: u32, vm: &mut dyn LuaVM) {
    compare(i, vm, Comparison::LUA_OPEQ);
}

// <
pub fn lt(i: u32, vm: &mut dyn LuaVM) {
    compare(i, vm, Comparison::LUA_OPLT);
}

// <=
pub fn le(i: u32, vm: &mut dyn LuaVM) {
    compare(i, vm, Comparison::LUA_OPLE);
}

// R(A) := not R(B)
pub fn not(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, mut b, _) = i.abc();
    a += 1;
    b += 1;

    vm.push_boolean(!vm.to_boolean(b));
    vm.replace(a);
}

// if (R(B) <=> C) then R(A) := R(B) else pc++
pub fn test_set(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, mut b, c) = i.abc();
    a += 1;
    b += 1;

    if vm.to_boolean(b) == (c != 0) {
        vm.copy(b, a);
    } else {
        vm.add_pc(1);
    }
}

// if not (R(A) <=> C) then pc++
pub fn test(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, _, c) = i.abc();
    a += 1;

    if vm.to_boolean(a) != (c != 0) {
        vm.add_pc(1);
    }
}
