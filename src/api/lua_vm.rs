use super::basic::LUA_REGISTRYINDEX;
pub use super::lua_api::LuaState as LuaAPI;

pub type RustFn = fn(&dyn super::lua_api::LuaState) -> usize;

pub trait LuaVM: LuaAPI {
    fn pc(&self) -> isize;
    fn add_pc(&mut self, n: isize);
    fn fetch(&mut self) -> u32;
    fn get_const(&mut self, idx: isize);
    fn get_rk(&mut self, rk: isize);
    fn register_count(&self) -> usize;
    fn load_vararg(&mut self, n: isize);
    fn load_proto(&mut self, idx: usize);
    fn stack_open(&self, s: &str);
    fn stack_closed(&self, s: &str);
    fn close_upvalues(&mut self, a: isize);
}

pub const fn lua_upvalue_index(i: isize) -> isize {
    LUA_REGISTRYINDEX - 1
}
