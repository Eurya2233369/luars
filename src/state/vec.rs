use core::panic;
use std::{cell::RefCell, rc::Rc};

use super::{closure::UpValue, lua_value::LuaValue};

pub trait MyVec<T> {
    fn set(&mut self, index: usize, val: T);
}

#[inline]
fn is_init<T>(v: &[T], i: usize) -> bool {
    v.get(i).is_some()
}

impl MyVec<Rc<RefCell<UpValue>>> for Vec<Rc<RefCell<UpValue>>> {
    fn set(&mut self, index: usize, val: Rc<RefCell<UpValue>>) {
        if is_init(self, index) {
            self[index] = val;
        } else {
            let n = index - self.len();
            let result = self.try_reserve(n);

            match result {
                Err(e) => panic!("error: {e}"),
                _ => {
                    if n > 0 {
                        for _ in 0..n - 1 {
                            self.push(Rc::new(RefCell::new(UpValue { val: LuaValue::Nil })));
                        }
                    }

                    self.push(val);
                }
            }
        }
    }
}
