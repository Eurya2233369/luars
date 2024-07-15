use crate::api::basic::Comparison;

use super::lua_value::LuaValue;

pub fn compare(a: &LuaValue, b: &LuaValue, op: Comparison) -> Option<bool> {
    match op {
        Comparison::LUA_OPEQ => Some(eq(a, b)),
        Comparison::LUA_OPLT => lt(a, b),
        Comparison::LUA_OPLE => le(a, b),
        _ => None,
    }
}

macro_rules! cmp {
    ($a:ident $op:tt $b:ident) => {
        match ($a, $b) {
            (LuaValue::Integer(x), LuaValue::Integer(y)) => Some(x $op y),
            (LuaValue::Integer(x), LuaValue::Number(y)) => Some((*x as f64) $op *y),
            (LuaValue::Number(x), LuaValue::Integer(y)) => Some(*x $op (*y as f64)),
            (LuaValue::Number(x), LuaValue::Number(y)) => Some(x $op y),
            (LuaValue::String(x), LuaValue::String(y)) => Some(x $op y),
            _ => None,
        }
    };
}

pub fn eq(a: &LuaValue, b: &LuaValue) -> bool {
    if let Some(x) = cmp!(a == b) {
        x
    } else {
        match a {
            LuaValue::Nil => matches!(b, LuaValue::Nil),
            LuaValue::Boolean(x) => match b {
                LuaValue::Boolean(y) => x == y,
                _ => false,
            },
            _ => false,
        }
    }
}

pub fn lt(a: &LuaValue, b: &LuaValue) -> Option<bool> {
    cmp!(a < b)
}

pub fn le(a: &LuaValue, b: &LuaValue) -> Option<bool> {
    cmp!(a <= b)
}
