use std::i64;

use crate::{
    api::basic::BasicType,
    number::{number, parser},
};

// copy ConstantType
#[derive(Debug, Clone)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
}

impl LuaValue {
    pub fn type_id(&self) -> BasicType {
        match self {
            Self::Nil => BasicType::LUA_TNIL,
            Self::Boolean(_) => BasicType::LUA_TBOOLEAN,
            Self::Integer(_) => BasicType::LUA_TNUMBER,
            Self::Number(_) => BasicType::LUA_TNUMBER,
            Self::String(_) => BasicType::LUA_TSTRING,

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
