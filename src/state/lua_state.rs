use crate::{
    api::{
        basic::{Arithmetic, BasicType, Comparison},
        lua_vm::{LuaAPI, LuaVM},
    },
    binary::chunk::{ConstantType, Prototype},
};

use super::{api_compare, lua_stack::LuaStack, lua_table::new_table, lua_value::LuaValue};

#[derive(Debug)]
pub struct LuaState {
    stack: LuaStack,
    proto: Prototype,
    pc: isize,
}

impl LuaState {
    pub fn new(stack_size: usize, proto: Prototype) -> Self {
        Self {
            stack: LuaStack::new(stack_size),
            proto: proto,
            pc: 0,
        }
    }

    pub fn pc(&self) -> isize {
        self.pc
    }
}

impl LuaAPI for LuaState {
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
    fn type_name_str(&self, tp: BasicType) -> &str {
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

    fn type_enum_id(&self, idx: isize) -> BasicType {
        if self.stack.is_valid(idx) {
            self.stack.get(idx).type_id()
        } else {
            BasicType::LUA_TNONE
        }
    }

    fn is_none(&self, idx: isize) -> bool {
        self.type_enum_id(idx) == BasicType::LUA_TNONE
    }

    fn is_nil(&self, idx: isize) -> bool {
        self.type_enum_id(idx) == BasicType::LUA_TNIL
    }

    fn is_none_or_nil(&self, idx: isize) -> bool {
        self.type_enum_id(idx).index() <= BasicType::LUA_TNIL.index()
    }

    fn is_boolean(&self, idx: isize) -> bool {
        self.type_enum_id(idx) == BasicType::LUA_TBOOLEAN
    }

    fn is_table(&self, idx: isize) -> bool {
        self.type_enum_id(idx) == BasicType::LUA_TTABLE
    }

    fn is_function(&self, idx: isize) -> bool {
        self.type_enum_id(idx) == BasicType::LUA_TFUNCTION
    }

    fn is_thread(&self, idx: isize) -> bool {
        self.type_enum_id(idx) == BasicType::LUA_TTHREAD
    }

    fn is_string(&self, idx: isize) -> bool {
        let t = self.type_enum_id(idx);

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
        let val = self.stack.get(idx);
        val.to_integer()
    }

    fn to_number(&self, idx: isize) -> f64 {
        self.to_numberx(idx).unwrap()
    }

    fn to_numberx(&self, idx: isize) -> Option<f64> {
        let val = self.stack.get(idx);
        val.to_float()
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

    fn arith(&mut self, op: Arithmetic) {
        if op != Arithmetic::LUA_OPUNM && op != Arithmetic::LUA_OPBNOT {
            let b = self.stack.pop();
            let a = self.stack.pop();
            if let Some(result) = super::api_arith::arith(&a, &b, &op) {
                self.stack.push(result);
                return;
            }
        } else {
            let a = self.stack.pop();
            if let Some(result) = super::api_arith::arith(&a, &a, &op) {
                self.stack.push(result);
                return;
            }
        }
        panic!("arithmetic error!");
    }

    fn compare(&self, idx1: isize, idx2: isize, op: Comparison) -> bool {
        if !self.stack.is_valid(idx1) || !self.stack.is_valid(idx2) {
            false
        } else {
            let a = self.stack.get(idx1);
            let b = self.stack.get(idx2);
            if let Some(result) = api_compare::compare(&a, &b, op) {
                return result;
            }
            panic!("comparison error!")
        }
    }

    fn len(&mut self, idx: isize) {
        let val = self.stack.get(idx);

        let len = match val {
            LuaValue::String(s) => s.len(),
            LuaValue::Table(t) => t.borrow().len(),
            _ => panic!("length error!"),
        };

        self.stack.push(LuaValue::Integer(len as i64));
    }

    fn concat(&mut self, n: isize) {
        if n == 0 {
            self.stack.push(LuaValue::String(String::new()));
        } else if n >= 2 {
            for _ in 1..n {
                if self.is_string(-1) && self.is_string(-2) {
                    let s2 = self.to_string(-1);
                    let mut s1 = self.to_string(-2);
                    s1.push_str(&s2);
                    self.stack.pop();
                    self.stack.pop();
                    self.stack.push(LuaValue::String(s1));
                } else {
                    println!("{:?}", self);
                    panic!("concatenation error!");
                }
            }
        }
    }

    /* get functions (Lua -> stack) */
    fn new_table(&mut self) {
        self.create_table(0, 0);
    }

    fn create_table(&mut self, n_arr: usize, n_rec: usize) {
        self.stack.push(new_table(n_arr, n_rec));
    }

    fn get_table(&mut self, idx: isize) -> BasicType {
        let t = self.stack.get(idx);
        let k = self.stack.pop();
        self.get_table_impl(&t, &k)
    }

    fn get_field(&mut self, idx: isize, k: &str) -> BasicType {
        let t = self.stack.get(idx);
        let k = LuaValue::String(k.to_string());
        // TODO
        self.get_table_impl(&t, &k)
    }

    fn get_i(&mut self, idx: isize, i: i64) -> BasicType {
        let t = self.stack.get(idx);
        let k = LuaValue::Integer(i);
        self.get_table_impl(&t, &k)
    }

    /* set functions (stack -> Lua) */
    fn set_table(&mut self, idx: isize) {
        let t = self.stack.get(idx);
        let v = self.stack.pop();
        let k = self.stack.pop();
        LuaState::set_table_impl(&t, k, v);
    }

    fn set_field(&mut self, idx: isize, k: &str) {
        let t = self.stack.get(idx);
        let v = self.stack.pop();
        let k = LuaValue::String(k.to_string());
        // TODO
        LuaState::set_table_impl(&t, k, v);
    }

    fn set_i(&mut self, idx: isize, i: i64) {
        let t = self.stack.get(idx);
        let v = self.stack.pop();
        let k = LuaValue::Integer(i);
        LuaState::set_table_impl(&t, k, v);
    }
}

impl LuaVM for LuaState {
    fn pc(&self) -> isize {
        self.pc
    }

    fn add_pc(&mut self, n: isize) {
        self.pc += n
    }

    fn fetch(&mut self) -> u32 {
        let i = self.proto.code()[self.pc as usize];
        self.pc += 1;
        i
    }

    fn get_const(&mut self, idx: isize) {
        let c = &self.proto.constants()[idx as usize];
        let val = match c {
            ConstantType::Nil => LuaValue::Nil,
            ConstantType::Boolean(b) => LuaValue::Boolean(*b),
            ConstantType::Integer(i) => LuaValue::Integer(*i),
            ConstantType::Number(n) => LuaValue::Number(*n),
            ConstantType::String(s) => LuaValue::String(s.clone()),
        };
        self.stack.push(val);
    }

    fn get_rk(&mut self, rk: isize) {
        if rk > 0xFF {
            // constant
            self.get_const(rk & 0xFF);
        } else {
            // register
            self.push_value(rk + 1);
        }
    }
}

impl LuaState {
    fn get_table_impl(&mut self, t: &LuaValue, k: &LuaValue) -> BasicType {
        if let LuaValue::Table(tbl) = t {
            let v = tbl.borrow().get(k);
            let type_id = v.type_id();
            self.stack.push(v);
            type_id
        } else {
            panic!("not a table!"); // TODO
        }
    }

    fn set_table_impl(t: &LuaValue, k: LuaValue, v: LuaValue) {
        if let LuaValue::Table(tbl) = t {
            tbl.borrow_mut().put(k, v);
        } else {
            panic!("not a table!");
        }
    }
}
