pub fn parse_integer(s: &str) -> Option<i64> {
    match s.parse::<i64>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

pub fn parse_float(s: &str) -> Option<f64> {
    match s.parse::<f64>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
