use super::{fpd::fb2int, instruction::Instruction};
use crate::api::lua_vm::LuaVM;

/* number of list items to accumulate before a SETLIST instruction */
const LFIELDS_PER_FLUSH: isize = 50;

// R(A) := {} (size = B,C)
pub fn new_table(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, b, c) = i.abc();

    a += 1;
    let n_arr = fb2int(b as usize);
    let n_rec = fb2int(c as usize);
    vm.create_table(n_arr, n_rec);
    vm.replace(a);
}

// R(A) := R(B)[RK(C)]
pub fn table(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, mut b, c) = i.abc();
    a += 1;
    b += 1;

    vm.get_rk(c);
    vm.table(b);
    vm.replace(a);
}

// R(A)[RK(B)] := RK(C)
pub fn set_table(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, b, c) = i.abc();
    a += 1;

    vm.get_rk(b);
    vm.get_rk(c);
    vm.set_table(a);
}

// R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B
pub fn set_list(i: u32, vm: &mut dyn LuaVM) {
    let (mut a, mut b, mut c) = i.abc();
    a += 1;

    if c > 0 {
        c -= 1;
    } else {
        c = vm.fetch().ax();
    }

    let b_is_zero = b == 0;
    if b_is_zero {
        b = vm.to_integer(-1) as isize - a - 1;
        vm.pop(1);
    }

    vm.check_stack(1);
    let mut idx = (c * LFIELDS_PER_FLUSH) as i64;
    for j in 1..(b + 1) {
        idx += 1;
        vm.push_value(a + j);
        vm.set_i(a, idx);
    }

    if b_is_zero {
        let nreg = vm.register_count() as isize;
        for j in (nreg + 1)..(vm.top() + 1) {
            idx += 1;
            vm.push_value(j);
            vm.set_i(a, idx);
        }

        // clear stack
        vm.set_top(nreg);
    }
}
