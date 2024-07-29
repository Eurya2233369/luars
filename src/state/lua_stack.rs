use std::{cell::RefCell, rc::Rc};

use crate::api::basic::LUA_REGISTRYINDEX;

use super::{
    closure::{Closure, UpValue},
    lua_value::LuaValue,
};

#[derive(Debug)]
pub struct LuaStack {
    pub slot: Vec<Rc<RefCell<LuaValue>>>,
    pub registry: LuaValue,
    pub closure: Rc<RefCell<Closure>>,
    pub openuvs: Vec<Rc<RefCell<UpValue>>>,
    pub varargs: Vec<Rc<RefCell<LuaValue>>>,
    pub pc: isize,
}

impl LuaStack {
    pub fn new(size: usize, registry: LuaValue, closure: Rc<RefCell<Closure>>) -> Self {
        Self {
            slot: Vec::with_capacity(size),
            registry,
            closure,
            openuvs: Vec::with_capacity(10),
            varargs: Vec::with_capacity(10),
            pc: 0,
        }
    }

    pub fn top(&self) -> isize {
        self.slot.len() as isize
    }

    pub fn check(&mut self, n: usize) {
        self.slot.reserve(n);
    }

    pub fn push(&mut self, val: Rc<RefCell<LuaValue>>) {
        self.slot.push(val);
    }

    pub fn push_n(&mut self, mut vals: Vec<Rc<RefCell<LuaValue>>>, n: isize) {
        vals.reverse();
        let nvals = vals.len();
        let un = if n < 0 { nvals } else { n as usize };

        for i in 0..un {
            if i < nvals {
                self.push(vals.pop().unwrap());
            } else {
                self.push(LuaValue::Nil.to_ptr());
            }
        }
    }

    pub fn pop(&mut self) -> Rc<RefCell<LuaValue>> {
        self.slot.pop().unwrap()
    }

    pub fn pop_n(&mut self, n: usize) -> Vec<Rc<RefCell<LuaValue>>> {
        let mut vec = Vec::with_capacity(n);
        for _ in 0..n {
            vec.push(self.pop());
        }
        vec.reverse();
        vec
    }

    pub fn set_top(&mut self, idx: isize) {
        let new_top = self.abs_index(idx);
        if new_top < 0 {
            panic!("stack underflow!");
        }

        let n = self.top() - new_top;
        match n.cmp(&0) {
            std::cmp::Ordering::Less => {
                for _ in n..0 {
                    self.push(LuaValue::Nil.to_ptr());
                }
            }
            std::cmp::Ordering::Equal => { /* ignored */ }
            std::cmp::Ordering::Greater => {
                for _ in 0..n {
                    self.pop();
                }
            }
        }
    }

    pub fn abs_index(&self, idx: isize) -> isize {
        if idx >= 0 || idx <= LUA_REGISTRYINDEX {
            idx
        } else {
            idx + self.top() + 1
        }
    }

    pub fn is_valid(&self, idx: isize) -> bool {
        if idx < LUA_REGISTRYINDEX {
            /* upvalues */
            let uv_idx = LUA_REGISTRYINDEX - idx - 1;
            let c = self.closure.borrow();
            return uv_idx < c.upvals.len() as isize;
        }

        if idx == LUA_REGISTRYINDEX {
            true
        } else {
            let abs_idx = self.abs_index(idx);
            abs_idx > 0 && abs_idx <= self.top()
        }
    }

    pub fn get(&self, idx: isize) -> Rc<RefCell<LuaValue>> {
        if idx < LUA_REGISTRYINDEX {
            /* upvalues */
            let uv_idx = LUA_REGISTRYINDEX - idx - 1;
            let c = self.closure.borrow_mut();

            if uv_idx >= c.upvals.len() as isize {
                return LuaValue::Nil.to_ptr();
            } else {
                return c.upvals[uv_idx as usize].borrow().val.clone();
            }
        }

        if idx == LUA_REGISTRYINDEX {
            self.registry.clone().to_ptr()
        } else {
            let abs_idx = self.abs_index(idx);
            if abs_idx > 0 && abs_idx <= self.top() {
                let idx = abs_idx as usize - 1;
                self.slot[idx].clone()
            } else {
                LuaValue::Nil.to_ptr()
            }
        }
    }

    pub fn set(&mut self, idx: isize, val: LuaValue) {
        if idx < LUA_REGISTRYINDEX {
            /* upvalues */
            let uv_idx = LUA_REGISTRYINDEX - idx - 1;
            let c = self.closure.borrow_mut();

            if uv_idx < c.upvals.len() as isize {
                let rc_upval = c.upvals[uv_idx as usize].borrow();
                let mut value = rc_upval.val.borrow_mut();
                *value = val;
            }
            return;
        }

        if idx == LUA_REGISTRYINDEX {
            self.registry = val;
            return;
        }

        let abs_idx = self.abs_index(idx);
        if abs_idx > 0 && abs_idx <= self.top() {
            let idx = abs_idx as usize - 1;

            let mut v = self.slot[idx].borrow_mut();
            *v = val;
        } else {
            panic!("invalid index!");
        }
    }

    pub fn reverse(&mut self, mut from: usize, mut to: usize) {
        while from < to {
            self.slot.swap(from, to);
            from += 1;
            to -= 1;
        }
    }
}
