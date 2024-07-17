use std::os::raw::c_double;

pub fn i_floor_div(a: i64, b: i64) -> i64 {
    if a > 0 && b > 0 || a < 0 && b < 0 || a % b == 0 {
        a / b
    } else {
        a / b - 1
    }
}

pub fn f_floor_div(a: f64, b: f64) -> f64 {
    (a / b).floor()
}

pub fn i_mod(a: i64, b: i64) -> i64 {
    a - i_floor_div(a, b) * b
}

extern "C" {
    fn fmod(x: c_double, y: c_double) -> c_double;
}

pub fn f_mod(a: f64, b: f64) -> f64 {
    unsafe { fmod(a, b) }
}

pub fn shift_left(a: i64, n: i64) -> i64 {
    a << n
}

pub fn shift_right(a: i64, n: i64) -> i64 {
    a >> n
}

pub fn float_to_integer(f: f64) -> Option<i64> {
    if f < i64::MIN as f64 || f > i64::MAX as f64 {
        None
    } else {
        Some(f.round() as i64)
    }
}
