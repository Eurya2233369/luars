use std::collections::HashMap;

use super::lua_value::LuaValue;

pub struct LuaTable {
    arr: Vec<LuaValue>,
    map: HashMap<LuaValue, LuaValue>,
}
