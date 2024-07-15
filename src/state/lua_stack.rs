use super::lua_value::LuaValue;

#[derive(Debug)]
pub struct LuaStack {
    slot: Vec<LuaValue>,
}

impl LuaStack {
    pub fn new(size: usize) -> Self {
        Self {
            slot: Vec::with_capacity(size),
        }
    }

    pub fn top(&self) -> isize {
        self.slot.len() as isize
    }

    pub fn check(&mut self, n: usize) {
        self.slot.reserve(n);
    }

    pub fn push(&mut self, val: LuaValue) {
        self.slot.push(val);
    }

    pub fn pop(&mut self) -> LuaValue {
        self.slot.pop().unwrap()
    }

    pub fn abs_index(&self, idx: isize) -> isize {
        if idx >= 0 {
            idx
        } else {
            idx + self.top() + 1
        }
    }

    pub fn is_valid(&self, idx: isize) -> bool {
        let abs_idx = self.abs_index(idx);
        abs_idx > 0 && abs_idx <= self.top()
    }

    pub fn get(&self, idx: isize) -> LuaValue {
        let abs_idx = self.abs_index(idx);

        if abs_idx > 0 && abs_idx <= self.top() {
            let idx = abs_idx as usize - 1;
            self.slot[idx].clone()
        } else {
            LuaValue::Nil
        }
    }

    pub fn set(&mut self, idx: isize, val: LuaValue) {
        let abs_idx = self.abs_index(idx);
        if abs_idx > 0 && abs_idx <= self.top() {
            let idx = abs_idx as usize - 1;
            self.slot[idx] = val;
        } else {
            panic!("invalid index!");
        }
    }

    pub fn reverse(&mut self, mut from: usize, mut to: usize) {
        while from < to {
            self.slot.swap(from, to);
            from += 1;
            to -= 1;
        }
    }
}
