use serde_json::Value;

use std::str::FromStr;

pub trait FromValue {
    fn new_from_value(value: Value) -> Self;
}

impl FromValue for String {
    fn new_from_value(value: Value) -> Self {
        String::from_str(value.as_str().unwrap_or("")).unwrap()
    }
}

impl FromValue for i32 {
    fn new_from_value(value: Value) -> Self {
        value.as_i64().unwrap_or(0) as i32
    }
}
