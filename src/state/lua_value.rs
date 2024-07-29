use core::fmt;
use std::{
    cell::RefCell, hash::{Hash, Hasher}, ptr, rc::Rc
};

use crate::{
    api::{basic::BasicType, lua_vm::RustFn},
    binary::chunk::Prototype,
    math::{number, parser},
};

use super::{closure::Closure, lua_table::LuaTable};

// copy ConstantType
#[derive(Clone)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
    Table(Rc<RefCell<LuaTable>>),
    Function(Rc<RefCell<Closure>>),
}

impl fmt::Debug for LuaValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuaValue::Nil => write!(f, "(nil)"),
            LuaValue::Boolean(b) => write!(f, "({})", b),
            LuaValue::Integer(i) => write!(f, "({})", i),
            LuaValue::Number(n) => write!(f, "({})", n),
            LuaValue::String(s) => write!(f, "({})", s),
            LuaValue::Table(_) => write!(f, "()"),
            LuaValue::Function(_) => write!(f, "(function)"),
        }
    }
}

impl PartialEq for LuaValue {
    fn eq(&self, other: &LuaValue) -> bool {
        if let (LuaValue::Nil, LuaValue::Nil) = (self, other) {
            true
        } else if let (LuaValue::Boolean(x), LuaValue::Boolean(y)) = (self, other) {
            x == y
        } else if let (LuaValue::Integer(x), LuaValue::Integer(y)) = (self, other) {
            x == y
        } else if let (LuaValue::Number(x), LuaValue::Number(y)) = (self, other) {
            x == y
        } else if let (LuaValue::String(x), LuaValue::String(y)) = (self, other) {
            x == y
        } else if let (LuaValue::Table(x), LuaValue::Table(y)) = (self, other) {
            Rc::ptr_eq(x, y)
        } else if let (LuaValue::Function(x), LuaValue::Function(y)) = (self, other) {
            Rc::ptr_eq(x, y)
        } else {
            false
        }
    }
}

impl Eq for LuaValue {}

impl Hash for LuaValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            LuaValue::Nil => 0.hash(state),
            LuaValue::Boolean(b) => b.hash(state),
            LuaValue::Integer(i) => i.hash(state),
            LuaValue::Number(n) => n.to_bits().hash(state),
            LuaValue::String(s) => s.hash(state),
            LuaValue::Table(t) => t.borrow().hash(state),
            LuaValue::Function(c) => c.borrow().hash(state),
        }
    }
}

impl LuaValue {
    pub fn type_id(&self) -> BasicType {
        match self {
            Self::Nil => BasicType::LUA_TNIL,
            Self::Boolean(_) => BasicType::LUA_TBOOLEAN,
            Self::Integer(_) => BasicType::LUA_TNUMBER,
            Self::Number(_) => BasicType::LUA_TNUMBER,
            Self::String(_) => BasicType::LUA_TSTRING,
            Self::Table(_) => BasicType::LUA_TTABLE,
            Self::Function(_) => BasicType::LUA_TFUNCTION,
            _ => todo!(),
        }
    }

    pub fn to_boolean(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Boolean(bool) => *bool,
            _ => true,
        }
    }

    pub fn to_float(&self) -> Option<f64> {
        match self {
            Self::Number(f) => Some(*f),
            Self::Integer(i) => Some(*i as f64),
            Self::String(s) => parser::parse_float(s),
            _ => None,
        }
    }

    pub fn to_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(i) => Some(*i),
            Self::Number(f) => number::float_to_integer(*f),
            Self::String(s) => Self::str_to_integer(s),
            _ => None,
        }
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    pub fn new_lua_fn(proto: Rc<Prototype>) -> Self {
        Self::Function(Rc::new(RefCell::new(Closure::new_lua_closure(proto))))
    }

    pub fn new_rust_fn(f: RustFn, n_upvals: usize) -> Self {
        Self::Function(Rc::new(RefCell::new(Closure::new_rust_closure(f, n_upvals))))
    }

    pub fn to_ptr(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    fn str_to_integer(s: &str) -> Option<i64> {
        let num = parser::parse_integer(s);
        if num.is_none() {
            match parser::parse_float(s) {
                Some(i) => number::float_to_integer(i),
                _ => None,
            }
        } else {
            num
        }
    }
}
