use serde_json::Value;

use std::str::FromStr;

use std::error::Error;

pub trait FromValue: Sized {
    fn new_from_value(value: Value) -> Result<Self, Box<Error>>;
}

impl FromValue for String {
    fn new_from_value(value: Value) -> Result<Self, Box<Error>> {
        match String::from_str(value.as_str().unwrap_or("")) {
            Ok(t) => Ok(t),
            Err(e) => Err(Box::new(e))
        }
    }
}

impl FromValue for i32 {
    fn new_from_value(value: Value) -> Result<Self, Box<Error>> {
        Ok(value.as_i64().unwrap_or(0) as i32)
    }
}
