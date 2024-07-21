use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    rc::Rc,
};

use uuid::Uuid;

use crate::binary::chunk::Prototype;

use super::lua_value::LuaValue;

#[derive(Debug)]
pub struct Closure {
    proto: Rc<Prototype>,
    uuid: Uuid,
}

impl Hash for Closure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}
impl Closure {
    pub fn new(proto: Rc<Prototype>) -> Self {
        Self {
            proto,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn proto(&self) -> &Rc<Prototype> {
        self.proto.borrow()
    }
}

pub fn new_lua_closure(proto: Rc<Prototype>) -> LuaValue {
    LuaValue::Function(Rc::new(Closure::new(proto)))
}
