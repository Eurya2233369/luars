use std::{cell::RefCell, rc::Rc};

use crate::{
    api::{
        basic::{Arithmetic, BasicType, Comparison, LUA_REGISTRYINDEX},
        lua_vm::{LuaAPI, LuaVM, RustFn},
    },
    binary::{
        self,
        chunk::{ConstantType, Prototype, Upvalue},
    },
    vm::instruction::Instruction,
};

use super::{
    api_compare,
    closure::{Closure, UpValue},
    lua_stack::LuaStack,
    lua_table::{self, new_table},
    lua_value::LuaValue,
    vec::MyVec,
};

const LUA_RIDX_GLOBALS: LuaValue = LuaValue::Integer(crate::api::basic::LUA_RIDX_GLOBALS as i64);

#[derive(Debug)]
pub struct LuaState {
    registry: LuaValue,
    frames: Vec<LuaStack>,
}

impl LuaState {
    pub fn new() -> Self {
        let registry = lua_table::new_table(0, 0);
        if let LuaValue::Table(t) = &registry {
            let globals = lua_table::new_table(0, 0);
            t.borrow_mut().put(LUA_RIDX_GLOBALS, globals);
        }

        let fake_proto = Rc::new(Prototype::new());
        let fake_closure = Rc::new(RefCell::new(Closure::new(fake_proto)));
        let fake_frame = LuaStack::new(20, registry.clone(), fake_closure);

        Self {
            registry,
            frames: vec![fake_frame],
        }
    }

    fn stack(&self) -> &LuaStack {
        self.frames.last().unwrap()
    }

    fn stack_mut(&mut self) -> &mut LuaStack {
        self.frames.last_mut().unwrap()
    }

    fn push_frame(&mut self, frame: LuaStack) {
        self.frames.push(frame);
    }

    fn pop_frame(&mut self) -> LuaStack {
        self.frames.pop().unwrap()
    }
}

impl LuaAPI for LuaState {
    fn top(&self) -> isize {
        self.stack().top()
    }

    fn abs_index(&self, idx: isize) -> isize {
        self.stack().abs_index(idx)
    }

    fn check_stack(&mut self, n: usize) -> bool {
        self.stack_mut().check(n);
        true
    }

    fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.stack_mut().pop();
        }
    }

    fn copy(&mut self, from_idx: isize, to_idx: isize) {
        let val = self.stack().get(from_idx);
        self.stack_mut().set(to_idx, val);
    }

    fn push_value(&mut self, idx: isize) {
        let val = self.stack().get(idx);
        self.stack_mut().push(val);
    }

    fn replace(&mut self, idx: isize) {
        let val = self.stack_mut().pop();
        self.stack_mut().set(idx, val);
    }

    fn insert(&mut self, idx: isize) {
        self.rotate(idx, 1);
    }

    fn remove(&mut self, idx: isize) {
        self.rotate(idx, -1);
        self.pop(1);
    }

    fn rotate(&mut self, idx: isize, n: isize) {
        let t = self.stack().top() - 1;
        let p = self.abs_index(idx) - 1;
        let m = if n >= 0 { t - n } else { p - n - 1 } as usize;
        {
            self.stack_mut().reverse(p as usize, m);
            self.stack_mut().reverse(m + 1, t as usize);
            self.stack_mut().reverse(p as usize, t as usize);
        }
    }

    fn set_top(&mut self, idx: isize) {
        let new_top = self.stack().abs_index(idx);
        let n = self.stack().top() - new_top;
        match n.cmp(&0) {
            std::cmp::Ordering::Greater => {
                for _ in 0..n {
                    self.stack_mut().pop();
                }
            }
            _ => {
                for _ in n..0 {
                    self.stack_mut().push(LuaValue::Nil);
                }
            }
        }
    }

    /* access functions (stack() -> rust) */
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
        if self.stack().is_valid(idx) {
            self.stack().get(idx).type_id()
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
        matches!(self.stack().get(idx), LuaValue::Integer(_))
    }

    fn is_rust_function(&self, idx: isize) -> bool {
        match self.stack().get(idx) {
            LuaValue::Function(c) => c.borrow().rust_fn().is_some(),
            _ => false,
        }
    }

    fn to_boolean(&self, idx: isize) -> bool {
        self.stack().get(idx).to_boolean()
    }

    fn to_integer(&self, idx: isize) -> i64 {
        self.to_integerx(idx).unwrap()
    }

    fn to_integerx(&self, idx: isize) -> Option<i64> {
        let val = self.stack().get(idx);
        val.to_integer()
    }

    fn to_number(&self, idx: isize) -> f64 {
        self.to_numberx(idx).unwrap()
    }

    fn to_numberx(&self, idx: isize) -> Option<f64> {
        let val = self.stack().get(idx);
        val.to_float()
    }

    fn to_string(&self, idx: isize) -> String {
        self.to_stringx(idx).unwrap()
    }

    fn to_stringx(&self, idx: isize) -> Option<String> {
        match self.stack().get(idx) {
            LuaValue::String(s) => Some(s),
            LuaValue::Number(n) => Some(n.to_string()),
            LuaValue::Integer(i) => Some(i.to_string()),
            _ => None,
        }
    }

    fn to_rust_function(&self, idx: isize) -> Option<RustFn> {
        match self.stack().get(idx) {
            LuaValue::Function(c) => c.borrow().rust_fn(),
            _ => None,
        }
    }

    /* push functions (rust -> stack()) */
    fn push_nil(&mut self) {
        self.stack_mut().push(LuaValue::Nil);
    }

    fn push_boolean(&mut self, b: bool) {
        self.stack_mut().push(LuaValue::Boolean(b));
    }

    fn push_integer(&mut self, n: i64) {
        self.stack_mut().push(LuaValue::Integer(n));
    }

    fn push_number(&mut self, n: f64) {
        self.stack_mut().push(LuaValue::Number(n));
    }

    fn push_string(&mut self, s: String) {
        self.stack_mut().push(LuaValue::String(s));
    }

    fn push_rust_fn(&mut self, f: RustFn) {
        self.stack_mut().push(LuaValue::new_rust_fn(f, 0));
    }

    fn push_global_table(&mut self) {
        if let LuaValue::Table(t) = &self.registry {
            let global = t.borrow().get(&LUA_RIDX_GLOBALS);
            self.stack_mut().push(global);
        }
    }

    fn push_rust_closure(&mut self, f: RustFn, n: isize) {
        let f = LuaValue::new_rust_fn(f, n as usize);
        if let LuaValue::Function(c) = &f {
            for i in (0..n).rev() {
                let val = self.stack_mut().pop();
                // c.borrow_mut().upvals[i as usize] = UpValue { val };
            }
        }

        self.stack_mut().push(f);
    }

    fn arith(&mut self, op: Arithmetic) {
        if op != Arithmetic::LUA_OPUNM && op != Arithmetic::LUA_OPBNOT {
            let b = self.stack_mut().pop();
            let a = self.stack_mut().pop();

            if let Some(result) = super::api_arith::arith(&a, &b, &op) {
                self.stack_mut().push(result);
                return;
            }
        } else {
            let a = self.stack_mut().pop();
            if let Some(result) = super::api_arith::arith(&a, &a, &op) {
                self.stack_mut().push(result);
                return;
            }
        }
        panic!("arithmetic error!");
    }

    fn compare(&self, idx1: isize, idx2: isize, op: Comparison) -> bool {
        if !self.stack().is_valid(idx1) || !self.stack().is_valid(idx2) {
            false
        } else {
            let a = self.stack().get(idx1);
            let b = self.stack().get(idx2);
            if let Some(result) = api_compare::compare(&a, &b, op) {
                return result;
            }
            panic!("comparison error!")
        }
    }

    fn len(&mut self, idx: isize) {
        let val = self.stack().get(idx);

        let len = match val {
            LuaValue::String(s) => s.len(),
            LuaValue::Table(t) => t.borrow().len(),
            _ => panic!("length error!"),
        };

        self.stack_mut().push(LuaValue::Integer(len as i64));
    }

    fn concat(&mut self, n: isize) {
        if n == 0 {
            self.stack_mut().push(LuaValue::String(String::new()));
        } else if n >= 2 {
            for _ in 1..n {
                if self.is_string(-1) && self.is_string(-2) {
                    let s2 = self.to_string(-1);
                    let mut s1 = self.to_string(-2);
                    s1.push_str(&s2);
                    self.stack_mut().pop();
                    self.stack_mut().pop();
                    self.stack_mut().push(LuaValue::String(s1));
                } else {
                    panic!("concatenation error!");
                }
            }
        }
    }

    /* get functions (Lua -> stack()) */
    fn new_table(&mut self) {
        self.create_table(0, 0);
    }

    fn create_table(&mut self, n_arr: usize, n_rec: usize) {
        self.stack_mut().push(new_table(n_arr, n_rec));
    }

    fn table(&mut self, idx: isize) -> BasicType {
        let t = self.stack().get(idx);
        let k = self.stack_mut().pop();
        self.get_table_impl(&t, &k)
    }

    fn field(&mut self, idx: isize, k: &str) -> BasicType {
        let t = self.stack().get(idx);
        let k = LuaValue::String(k.to_string());
        // TODO
        self.get_table_impl(&t, &k)
    }

    fn i(&mut self, idx: isize, i: i64) -> BasicType {
        let t = self.stack().get(idx);
        let k = LuaValue::Integer(i);
        self.get_table_impl(&t, &k)
    }

    fn global(&mut self, name: &str) -> BasicType {
        if let LuaValue::Table(r) = &self.registry {
            let t = r.borrow().get(&LUA_RIDX_GLOBALS);
            let k = LuaValue::String(name.to_string()); // TODO
            self.get_table_impl(&t, &k)
        } else {
            BasicType::LUA_TNONE
        }
    }

    /* set functions (stack() -> Lua) */
    fn set_table(&mut self, idx: isize) {
        let t = self.stack().get(idx);
        let v = self.stack_mut().pop();
        let k = self.stack_mut().pop();
        LuaState::set_table_impl(&t, k, v);
    }

    fn set_field(&mut self, idx: isize, k: &str) {
        let t = self.stack().get(idx);
        let v = self.stack_mut().pop();
        let k = LuaValue::String(k.to_string());
        // TODO
        LuaState::set_table_impl(&t, k, v);
    }

    fn set_i(&mut self, idx: isize, i: i64) {
        let t = self.stack().get(idx);
        let v = self.stack_mut().pop();
        let k = LuaValue::Integer(i);
        LuaState::set_table_impl(&t, k, v);
    }

    fn set_global(&mut self, name: &str) {
        if let LuaValue::Table(r) = &self.registry {
            let t = r.borrow().get(&LUA_RIDX_GLOBALS);
            let v = self.stack_mut().pop();
            let k = LuaValue::String(name.to_string()); // TODO
            LuaState::set_table_impl(&t, k, v);
        }
    }

    fn register(&mut self, name: &str, f: RustFn) {
        self.push_rust_fn(f);
        self.set_global(name);
    }

    /* 'load' and 'call' functions (load and run Lua code) */
    fn load(&mut self, chunk: Vec<u8>, chunk_name: &str, mode: &str) -> u8 {
        let proto = binary::un_dump(chunk);
        let size = proto.upvalues().len();
        let f = LuaValue::new_lua_fn(proto);

        if let LuaValue::Function(c) = &f {
            if size > 0 {
                let mut closure = c.borrow_mut();

                if let LuaValue::Table(t) = &self.registry {
                    let env = t.borrow().get(&LUA_RIDX_GLOBALS);
                    let upvals = &mut closure.upvals;
                    upvals.set(0, Rc::new(RefCell::new(UpValue { val: env })));
                }
            }
        }

        self.stack_mut().push(f);

        0 // TODO
    }

    fn call(&mut self, n_args: usize, n_results: isize) {
        let val = self.stack().get(-(n_args as isize + 1));
        if let LuaValue::Function(c) = val {
            if c.borrow().rust_fn().is_some() {
                self.call_rust_closure(n_args, n_results, c)
            } else {
                self.call_lua_closure(n_args, n_results, c);
            }
        } else {
            panic!("not function!");
        }
    }
}

impl LuaVM for LuaState {
    fn pc(&self) -> isize {
        self.stack().pc
    }

    fn add_pc(&mut self, n: isize) {
        self.stack_mut().pc += n
    }

    fn fetch(&mut self) -> u32 {
        let i = self.stack().closure.borrow().proto().code()[self.stack().pc as usize];
        self.stack_mut().pc += 1;
        i
    }

    fn get_const(&mut self, idx: isize) {
        let c = {
            let closure = &self.stack().closure.borrow();
            closure.proto().constants()[idx as usize].clone()
        };

        let val = match c {
            ConstantType::Nil => LuaValue::Nil,
            ConstantType::Boolean(b) => LuaValue::Boolean(b),
            ConstantType::Integer(i) => LuaValue::Integer(i),
            ConstantType::Number(n) => LuaValue::Number(n),
            ConstantType::String(s) => LuaValue::String(s.clone()),
        };

        self.stack_mut().push(val);
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

    fn register_count(&self) -> usize {
        self.stack().closure.borrow().proto().max_stack_size() as usize
    }

    fn load_vararg(&mut self, mut n: isize) {
        if n < 0 {
            n = self.stack().varargs.len() as isize;
        }

        let varargs = self.stack().varargs.clone();
        self.stack_mut().check(n as usize);
        self.stack_mut().push_n(varargs, n);
    }

    fn load_proto(&mut self, idx: usize) {
        let proto = {
            let closure = self.stack().closure.borrow();
            closure.proto().protos()[idx].clone()
        };

        let f = LuaValue::new_lua_fn(proto.clone());

        if let LuaValue::Function(c) = &f {
            let mut closure = c.borrow_mut();
            let stack = self.stack_mut();

            for (i, uv_info) in proto.upvalues().iter().enumerate() {
                let uv_idx = uv_info.idx as usize;
                match uv_info.instack.cmp(&1) {
                    std::cmp::Ordering::Equal => {
                        if let Some(open_uv) = stack.openuvs.get(i) {
                            closure.upvals.set(i, open_uv.clone());
                        } else {
                            let val = Rc::new(RefCell::new(UpValue {
                                val: stack.slot[uv_idx].clone(),
                            }));
                            closure.upvals.set(i, val.clone());
                            stack.openuvs.set(i, val);
                        }
                    }
                    _ => closure
                        .upvals
                        .set(i, stack.closure.borrow_mut().upvals[uv_idx].clone()),
                }
            }
        }

        self.stack_mut().push(f);
    }

    fn stack_open(&self, s: &str) {
        println!("{s} open {:?}", self.stack());
    }

    fn stack_closed(&self, s: &str) {
        println!("{s} closed {:?}", self.stack());
    }

    fn close_upvalues(&mut self, a: isize) {
        let openuvs = &mut self.stack_mut().openuvs;

        let indices_to_remove: Vec<usize> = openuvs
            .iter()
            .enumerate()
            .filter(|(i, _)| *i as isize >= a - 1)
            .map(|(i, _)| i)
            .collect();

        for i in indices_to_remove.iter().rev() {
            println!("close_upvalues {:?}", openuvs.remove(*i));
        }
    }
}

impl LuaState {
    fn get_table_impl(&mut self, t: &LuaValue, k: &LuaValue) -> BasicType {
        if let LuaValue::Table(tbl) = t {
            let v = tbl.borrow().get(k);
            let type_id = v.type_id();
            self.stack_mut().push(v);
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

    fn call_rust_closure(&mut self, nargs: usize, nresults: isize, c: Rc<RefCell<Closure>>) {
        let rust_fn = c.borrow().rust_fn().unwrap();
        let mut new_stack = LuaStack::new(nargs + 20, self.registry.clone(), c);

        if nargs > 0 {
            let args = self.stack_mut().pop_n(nargs);
            new_stack.push_n(args, nargs as isize);
        }

        self.stack_mut().pop();
        self.push_frame(new_stack);
        let r = rust_fn(self);
        new_stack = self.pop_frame();

        if nresults != 0 {
            let results = new_stack.pop_n(r);
            self.stack_mut().check(results.len());
            self.stack_mut().push_n(results, nresults);
        }
    }

    fn call_lua_closure(&mut self, n_args: usize, n_results: isize, c: Rc<RefCell<Closure>>) {
        let n_regs = c.borrow().proto().max_stack_size() as usize;
        let n_params = c.borrow().proto().num_params() as usize;
        let is_vararg = c.borrow().proto().is_vararg() == 1;

        let mut new_stack = LuaStack::new(n_regs + 20, self.registry.clone(), c);
        let mut args = self.stack_mut().pop_n(n_args);
        self.stack_mut().pop();
        if n_args > n_params {
            for _ in n_params..n_args {
                new_stack.varargs.push(args.pop().unwrap());
            }
            if is_vararg {
                new_stack.varargs.reverse();
            } else {
                new_stack.varargs.clear();
            }
        }
        new_stack.push_n(args, n_params as isize);
        new_stack.set_top(n_regs as isize);

        self.push_frame(new_stack);
        self.run_lua_closure();
        new_stack = self.pop_frame();

        if n_results != 0 {
            let n_rets = new_stack.top() as usize - n_regs;
            let results = new_stack.pop_n(n_rets);
            self.stack_mut().check(n_rets);
            self.stack_mut().push_n(results, n_results);
        }
    }

    fn run_lua_closure(&mut self) {
        loop {
            let instr = self.fetch();
            instr.execute(self);
            if instr.opcode() == crate::vm::opcode::OP_RETURN {
                break;
            }
        }
    }
}
