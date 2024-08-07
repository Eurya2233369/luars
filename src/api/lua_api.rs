use super::{
    basic::{Arithmetic, BasicType, Comparison},
    lua_vm::RustFn,
};

pub trait LuaState {
    fn top(&self) -> isize;
    fn abs_index(&self, idx: isize) -> isize;
    fn check_stack(&mut self, n: usize) -> bool;
    fn pop(&mut self, n: usize);
    fn copy(&mut self, from_idx: isize, to_idx: isize);
    fn push_value(&mut self, idx: isize);
    fn replace(&mut self, idx: isize);
    fn insert(&mut self, idx: isize);
    fn remove(&mut self, idx: isize);
    fn rotate(&mut self, idx: isize, n: isize);
    fn set_top(&mut self, idx: isize);
    /* access functions (stack -> rust) */
    fn type_name_str(&self, tp: BasicType) -> &str;
    fn type_enum_id(&self, idx: isize) -> BasicType;
    fn is_none(&self, idx: isize) -> bool;
    fn is_nil(&self, idx: isize) -> bool;
    fn is_none_or_nil(&self, idx: isize) -> bool;
    fn is_boolean(&self, idx: isize) -> bool;
    fn is_integer(&self, idx: isize) -> bool;
    fn is_number(&self, idx: isize) -> bool;
    fn is_string(&self, idx: isize) -> bool;
    fn is_table(&self, idx: isize) -> bool;
    fn is_thread(&self, idx: isize) -> bool;
    fn is_function(&self, idx: isize) -> bool;
    fn is_rust_function(&self, idx: isize) -> bool;
    fn to_boolean(&self, idx: isize) -> bool;
    fn to_integer(&self, idx: isize) -> i64;
    fn to_integerx(&self, idx: isize) -> Option<i64>;
    fn to_number(&self, idx: isize) -> f64;
    fn to_numberx(&self, idx: isize) -> Option<f64>;
    fn to_string(&self, idx: isize) -> String;
    fn to_stringx(&self, idx: isize) -> Option<String>;
    fn to_rust_function(&self, idx: isize) -> Option<RustFn>;
    /* push functions (rust -> stack) */
    fn push_nil(&mut self);
    fn push_boolean(&mut self, b: bool);
    fn push_integer(&mut self, n: i64);
    fn push_number(&mut self, n: f64);
    fn push_string(&mut self, s: String);
    fn push_rust_fn(&mut self, f: RustFn);
    fn push_global_table(&mut self);
    fn push_rust_closure(&mut self, f: RustFn, n: isize);
    /* comparison and arithmetic functions */
    fn arith(&mut self, op: Arithmetic);
    fn compare(&self, idx1: isize, idx2: isize, op: Comparison) -> bool;
    /* miscellaneous functions */
    fn len(&mut self, idx: isize);
    fn concat(&mut self, n: isize);
    /* ger functions (rust -> stack) */
    fn new_table(&mut self);
    fn create_table(&mut self, n_arr: usize, n_rec: usize);
    fn table(&mut self, idx: isize) -> BasicType;
    fn field(&mut self, idx: isize, k: &str) -> BasicType;
    fn i(&mut self, idx: isize, i: i64) -> BasicType;
    fn global(&mut self, name: &str) -> BasicType;
    /* set functions (stack -> Lua) */
    fn set_table(&mut self, idx: isize);
    fn set_field(&mut self, idx: isize, k: &str);
    fn set_i(&mut self, idx: isize, i: i64);
    fn set_global(&mut self, name: &str);
    fn register(&mut self, name: &str, f: RustFn);
    /* 'load' and 'call' functions (load and run Lua code) */
    fn load(&mut self, chunk: Vec<u8>, chunk_name: &str, mode: &str) -> u8;
    fn call(&mut self, n_args: usize, n_results: isize);
}
