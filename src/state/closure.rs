use std::{
    borrow::Borrow,
    cell::RefCell,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::{api::lua_vm::RustFn, binary::chunk::Prototype};

use super::lua_value::LuaValue;

#[derive(Debug)]
pub struct Closure {
    proto: Rc<Prototype>,
    rust_fn: Option<RustFn>,
    pub upvals: Vec<Rc<RefCell<UpValue>>>,
    pub address: usize,
}

impl Hash for Closure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}
impl Closure {
    pub fn new(proto: Rc<Prototype>) -> Self {
        let mut this = Self {
            proto,
            rust_fn: None,
            upvals: vec![],
            address: 0,
        };
        this.address = std::ptr::addr_of!(this) as usize;
        this
    }

    pub fn new_lua_closure(proto: Rc<Prototype>) -> Self {
        let upvals = {
            let size = proto.upvalues().len();
            Vec::with_capacity(size)
        };

        let mut this = Self::new(proto);
        this.upvals = upvals;
        this
    }

    pub fn new_rust_closure(f: RustFn, n_upvals: usize) -> Self {
        let mut this = Self::new(Rc::new(Prototype::new()));
        this.rust_fn = Some(f);
        this.upvals = Vec::with_capacity(n_upvals);
        this
    }

    pub fn proto(&self) -> &Rc<Prototype> {
        self.proto.borrow()
    }

    pub fn rust_fn(&self) -> Option<RustFn> {
        self.rust_fn
    }
}

#[derive(Debug, Clone)]
pub struct UpValue {
    pub val: LuaValue,
}
