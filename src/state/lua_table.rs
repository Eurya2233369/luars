use std::{
    cell::RefCell,
    collections::HashMap,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::math::number;

use super::lua_value::LuaValue;

#[derive(Clone)]
pub struct LuaTable {
    arr: Vec<LuaValue>,
    map: HashMap<LuaValue, LuaValue>,
    address: usize,
}

impl Hash for LuaTable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

impl LuaTable {
    pub fn new(n_arr: usize, n_rec: usize) -> Self {
        let mut this = Self {
            arr: Vec::with_capacity(n_arr),
            map: HashMap::with_capacity(n_rec),
            address: 0,
        };
        this.address = std::ptr::addr_of!(this) as usize;
        this
    }

    pub fn get(&self, key: &LuaValue) -> LuaValue {
        //TODO
        if let Some(i) = Self::to_integer(key) {
            if i <= self.arr.len() {
                return self.arr[i - 1].clone();
            }
        }

        // TODO
        if let Some(val) = self.map.get(key) {
            val.clone()
        } else {
            LuaValue::Nil
        }
    }

    pub fn put(&mut self, key: LuaValue, value: LuaValue) {
        if key.is_nil() {
            panic!("table index is nil!");
        }

        if let LuaValue::Number(n) = key {
            if n.is_nan() {
                panic!("table index is NaN!");
            }
        }

        if let Some(idx) = Self::to_integer(&key) {
            let arr_len = self.arr.len();
            if idx <= arr_len {
                let is_nil = value.is_nil();

                self.arr[idx - 1] = value;
                if idx == arr_len && !is_nil {
                    self.shrink_array();
                }
                return;
            }

            if idx == arr_len + 1 {
                self.map.remove(&key);
                if !value.is_nil() {
                    self.arr.push(value);
                    self.expand_array();
                }
                return;
            }
        }

        if !value.is_nil() {
            self.map.insert(key, value);
        } else {
            self.map.remove(&key);
        }
    }

    pub fn len(&self) -> usize {
        self.arr.len()
    }

    fn shrink_array(&mut self) {
        while !self.arr.is_empty() {
            if self.arr.last().unwrap().is_nil() {
                self.arr.pop();
            } else {
                break;
            }
        }
    }

    fn expand_array(&mut self) {
        let mut idx = self.arr.len() + 1;
        loop {
            let key = LuaValue::Integer(idx as i64);
            if self.map.contains_key(&key) {
                let val = self.map.remove(&key).unwrap();
                self.arr.push(val);
                idx += 1;
            } else {
                break;
            }
        }
    }

    fn to_integer(key: &LuaValue) -> Option<usize> {
        match key {
            LuaValue::Integer(i) if *i >= 1 => Some(*i as usize),
            LuaValue::Number(n) => match number::float_to_integer(*n) {
                Some(i) if i >= 1 => Some(i as usize),
                _ => None,
            },
            _ => None,
        }
    }
}

pub fn new_table(n_arr: usize, n_rec: usize) -> LuaValue {
    LuaValue::Table(Rc::new(RefCell::new(LuaTable::new(n_arr, n_rec))))
}
