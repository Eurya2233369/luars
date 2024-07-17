use crate::{
    api::basic::Arithmetic,
    math::number::{f_floor_div, f_mod, i_floor_div, i_mod, shift_left, shift_right},
};

use super::lua_value::LuaValue;

fn iadd(a: i64, b: i64) -> i64 {
    a + b
}
fn fadd(a: f64, b: f64) -> f64 {
    a + b
}
fn isub(a: i64, b: i64) -> i64 {
    a - b
}
fn fsub(a: f64, b: f64) -> f64 {
    a - b
}
fn imul(a: i64, b: i64) -> i64 {
    a * b
}
fn fmul(a: f64, b: f64) -> f64 {
    a * b
}
fn imod(a: i64, b: i64) -> i64 {
    i_mod(a, b)
}
fn fmod(a: f64, b: f64) -> f64 {
    f_mod(a, b)
}
fn pow(a: f64, b: f64) -> f64 {
    a.powf(b)
}
fn div(a: f64, b: f64) -> f64 {
    a / b
}
fn iidiv(a: i64, b: i64) -> i64 {
    i_floor_div(a, b)
}
fn fidiv(a: f64, b: f64) -> f64 {
    f_floor_div(a, b)
}
fn band(a: i64, b: i64) -> i64 {
    a & b
}
fn bor(a: i64, b: i64) -> i64 {
    a | b
}
fn bxor(a: i64, b: i64) -> i64 {
    a ^ b
}
fn shl(a: i64, b: i64) -> i64 {
    shift_left(a, b)
}
fn shr(a: i64, b: i64) -> i64 {
    shift_right(a, b)
}
fn iunm(a: i64, _: i64) -> i64 {
    -a
}
fn funm(a: f64, _: f64) -> f64 {
    -a
}
fn bnot(a: i64, _: i64) -> i64 {
    !a
}

fn inone(_: i64, _: i64) -> i64 {
    0
}
fn fnone(_: f64, _: f64) -> f64 {
    0.0
}

type IAddress = fn(i64, i64) -> i64;
type FAddress = fn(f64, f64) -> f64;

pub const OPS: &'static [(IAddress, FAddress)] = &[
    (iadd, fadd),
    (isub, fsub),
    (imul, fmul),
    (imod, fmod),
    (inone, pow),
    (inone, div),
    (iidiv, fidiv),
    (band, fnone),
    (bor, fnone),
    (bxor, fnone),
    (shl, fnone),
    (shr, fnone),
    (iunm, funm),
    (bnot, fnone),
];

pub const NONE: &'static [(IAddress, FAddress); 1] = &[(inone, fnone)];

pub fn arith(a: &LuaValue, b: &LuaValue, op: &Arithmetic) -> Option<LuaValue> {
    let iop = OPS[op.index() as usize].0;
    let fop = OPS[op.index() as usize].1;
    if fop == NONE[0].1 {
        // bitwise
        if let Some(x) = a.to_integer() {
            if let Some(y) = b.to_integer() {
                return Some(LuaValue::Integer(iop(x, y)));
            }
        }
    } else {
        // arith
        if iop != NONE[0].0 {
            // add,sub,mul,mod,idiv,unm
            if let LuaValue::Integer(x) = a {
                if let LuaValue::Integer(y) = b {
                    return Some(LuaValue::Integer(iop(*x, *y)));
                }
            }
        }
        if let Some(x) = a.to_float() {
            if let Some(y) = b.to_float() {
                return Some(LuaValue::Number(fop(x, y)));
            }
        }
    }
    None
}
