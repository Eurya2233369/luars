use std::rc::Rc;

use super::{closure::Closure, lua_value::LuaValue};

#[derive(Debug)]
pub struct LuaStack {
    slot: Vec<LuaValue>,
    pub closure: Rc<Closure>,
    pub varargs: Vec<LuaValue>,
    pub pc: isize,
}

impl LuaStack {
    pub fn new(size: usize, closure: Rc<Closure>) -> Self {
        Self {
            slot: Vec::with_capacity(size),
            closure,
            varargs: Vec::new(),
            pc: 0,
        }
    }

    pub fn top(&self) -> isize {
        self.slot.len() as isize
    }

    pub fn check(&mut self, n: usize) {
        self.slot.reserve(n);
    }

    pub fn push(&mut self, val: LuaValue) {
        self.slot.push(val);
    }

    pub fn push_n(&mut self, mut vals: Vec<LuaValue>, n: isize) {
        vals.reverse();
        let nvals = vals.len();
        let un = if n < 0 { nvals } else { n as usize };

        for i in 0..un {
            if i < nvals {
                self.push(vals.pop().unwrap());
            } else {
                self.push(LuaValue::Nil);
            }
        }
    }

    pub fn pop(&mut self) -> LuaValue {
        self.slot.pop().unwrap()
    }

    pub fn pop_n(&mut self, n: usize) -> Vec<LuaValue> {
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
                    self.push(LuaValue::Nil);
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
        if idx >= 0 {
            idx
        } else {
            idx + self.top() + 1
        }
    }

    pub fn is_valid(&self, idx: isize) -> bool {
        let abs_idx = self.abs_index(idx);
        abs_idx > 0 && abs_idx <= self.top()
    }

    pub fn get(&self, idx: isize) -> LuaValue {
        let abs_idx = self.abs_index(idx);

        if abs_idx > 0 && abs_idx <= self.top() {
            let idx = abs_idx as usize - 1;
            self.slot[idx].clone()
        } else {
            LuaValue::Nil
        }
    }

    pub fn set(&mut self, idx: isize, val: LuaValue) {
        let abs_idx = self.abs_index(idx);
        if abs_idx > 0 && abs_idx <= self.top() {
            let idx = abs_idx as usize - 1;
            self.slot[idx] = val;
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
