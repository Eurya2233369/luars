mod api_arith;
mod api_compare;
mod closure;
mod lua_stack;
mod lua_state;
mod lua_table;
mod lua_value;
mod vec;

pub use self::lua_state::LuaState;

pub fn new_lua_state() -> LuaState {
    LuaState::new()
}
