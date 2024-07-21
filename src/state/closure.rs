use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::{api::lua_vm::RustFn, binary::chunk::Prototype};

#[derive(Debug)]
pub struct Closure {
    proto: Rc<Prototype>,
    rust_fn: Option<RustFn>,
    address: usize,
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
            address: 0,
        };
        this.address = std::ptr::addr_of!(this) as usize;
        this
    }

    pub fn new_lua_closure(proto: Rc<Prototype>) -> Self {
        Self::new(proto)
    }

    pub fn new_rust_closure(f: RustFn) -> Self {
        let mut this = Self::new(Rc::new(Prototype::new()));
        this.rust_fn = Some(f);
        this
    }

    pub fn proto(&self) -> &Rc<Prototype> {
        self.proto.borrow()
    }

    pub fn rust_fn(&self) -> Option<RustFn> {
        self.rust_fn
    }
}
