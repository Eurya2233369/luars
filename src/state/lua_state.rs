use crate::api::{basic::BasicType, lua_api};

use super::{lua_stack::LuaStack, lua_value::LuaValue};

pub struct LuaState {
    stack: LuaStack,
}

impl LuaState {
    pub fn new() -> Self {
        Self {
            stack: LuaStack::new(20),
        }
    }
}

impl lua_api::LuaState for LuaState {
    fn top(&self) -> isize {
        self.stack.top()
    }

    fn abs_index(&self, idx: isize) -> isize {
        self.stack.abs_index(idx)
    }

    fn check_stack(&mut self, n: usize) -> bool {
        self.stack.check(n);
        true
    }

    fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.stack.pop();
        }
    }

    fn copy(&mut self, from_idx: isize, to_idx: isize) {
        let val = self.stack.get(from_idx);
        self.stack.set(to_idx, val);
    }

    fn push_value(&mut self, idx: isize) {
        let val = self.stack.get(idx);
        self.stack.push(val);
    }

    fn replace(&mut self, idx: isize) {
        let val = self.stack.pop();
        self.stack.set(idx, val);
    }

    fn insert(&mut self, idx: isize) {
        self.rotate(idx, 1);
    }

    fn remove(&mut self, idx: isize) {
        self.rotate(idx, -1);
        self.pop(1);
    }

    fn rotate(&mut self, idx: isize, n: isize) {
        let t = self.stack.top() - 1;
        let p = self.abs_index(idx) - 1;
        let m = if n >= 0 { t - n } else { p - n - 1 } as usize;
        {
            self.stack.reverse(p as usize, m);
            self.stack.reverse(m + 1, t as usize);
            self.stack.reverse(p as usize, t as usize);
        }
    }

    fn set_top(&mut self, idx: isize) {
        let new_top = self.stack.abs_index(idx);
        let n = self.stack.top() - new_top;
        match n.cmp(&0) {
            std::cmp::Ordering::Greater => {
                for _ in 0..n {
                    self.stack.pop();
                }
            }
            _ => {
                for _ in n..0 {
                    self.stack.push(LuaValue::Nil);
                }
            }
        }
    }

    /* access functions (stack -> rust) */
    fn type_name(&self, tp: BasicType) -> &str {
        match tp {
            BasicType::LUA_TNONE => "no value",
            BasicType::LUA_TNIL => "nil",
            BasicType::LUA_TBOOLEAN => "boolean",
            BasicType::LUA_TNUMBER => "number",
            BasicType::LUA_TSTRING => "string",
            BasicType::LUA_TTABLE => "table",
            BasicType::LUA_TFUNCTION => "function",
            BasicType::LUA_TTHREAD => "thread",
            _ => "userdata",
        }
    }

    fn type_id(&self, idx: isize) -> BasicType {
        if self.stack.is_valid(idx) {
            self.stack.get(idx).type_id()
        } else {
            BasicType::LUA_TNONE
        }
    }

    fn is_none(&self, idx: isize) -> bool {
        self.type_id(idx) == BasicType::LUA_TNONE
    }

    fn is_nil(&self, idx: isize) -> bool {
        self.type_id(idx) == BasicType::LUA_TNIL
    }

    fn is_none_or_nil(&self, idx: isize) -> bool {
        self.type_id(idx).index() <= BasicType::LUA_TNIL.index()
    }

    fn is_boolean(&self, idx: isize) -> bool {
        self.type_id(idx) == BasicType::LUA_TBOOLEAN
    }

    fn is_table(&self, idx: isize) -> bool {
        self.type_id(idx) == BasicType::LUA_TTABLE
    }

    fn is_function(&self, idx: isize) -> bool {
        self.type_id(idx) == BasicType::LUA_TFUNCTION
    }

    fn is_thread(&self, idx: isize) -> bool {
        self.type_id(idx) == BasicType::LUA_TTHREAD
    }

    fn is_string(&self, idx: isize) -> bool {
        let t = self.type_id(idx);
        t == BasicType::LUA_TSTRING || t == BasicType::LUA_TNUMBER
    }

    fn is_number(&self, idx: isize) -> bool {
        self.to_numberx(idx).is_some()
    }

    fn is_integer(&self, idx: isize) -> bool {
        matches!(self.stack.get(idx), LuaValue::Integer(_))
    }

    fn to_boolean(&self, idx: isize) -> bool {
        self.stack.get(idx).to_boolean()
    }

    fn to_integer(&self, idx: isize) -> i64 {
        self.to_integerx(idx).unwrap()
    }

    fn to_integerx(&self, idx: isize) -> Option<i64> {
        match self.stack.get(idx) {
            LuaValue::Integer(i) => Some(i),
            _ => None,
        }
    }

    fn to_number(&self, idx: isize) -> f64 {
        self.to_numberx(idx).unwrap()
    }

    fn to_numberx(&self, idx: isize) -> Option<f64> {
        match self.stack.get(idx) {
            LuaValue::Number(n) => Some(n),
            LuaValue::Integer(i) => Some(i as f64),
            _ => None,
        }
    }

    fn to_string(&self, idx: isize) -> String {
        self.to_stringx(idx).unwrap()
    }

    fn to_stringx(&self, idx: isize) -> Option<String> {
        match self.stack.get(idx) {
            LuaValue::String(s) => Some(s),
            LuaValue::Number(n) => Some(n.to_string()),
            LuaValue::Integer(i) => Some(i.to_string()),
            _ => None,
        }
    }

    /* push functions (rust -> stack) */
    fn push_nil(&mut self) {
        self.stack.push(LuaValue::Nil);
    }

    fn push_boolean(&mut self, b: bool) {
        self.stack.push(LuaValue::Boolean(b));
    }

    fn push_integer(&mut self, n: i64) {
        self.stack.push(LuaValue::Integer(n));
    }

    fn push_number(&mut self, n: f64) {
        self.stack.push(LuaValue::Number(n));
    }

    fn push_string(&mut self, s: String) {
        self.stack.push(LuaValue::String(s));
    }
}
