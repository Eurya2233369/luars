use crate::api::basic::BasicType;

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
}
