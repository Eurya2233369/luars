mod closure;
mod lua_stack;
mod lua_state;
mod lua_table;
mod lua_value;
// mod api_stack;
mod api_arith;
// mod api_call;
mod api_compare;

pub use self::lua_state::LuaState;

pub fn new_lua_state() -> LuaState {
    LuaState::new()
}
