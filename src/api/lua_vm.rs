pub use super::lua_api::LuaState as LuaAPI;

pub trait LuaVM: LuaAPI {
    fn pc(&self) -> isize;
    fn add_pc(&mut self, n: isize);
    fn fetch(&mut self) -> u32;
    fn get_const(&mut self, idx: isize);
    fn get_rk(&mut self, rk: isize);
}
