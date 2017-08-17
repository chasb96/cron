use ::from_value::FromValue;
use ::run::{ Runnable, RunSuccess, RunError };

use serde_json::Value;

pub struct EmptyJob {

}

impl EmptyJob {
    pub fn new() -> Self {
        EmptyJob { }
    }
}

impl FromValue for EmptyJob {
    fn new_from_value(_: Value) -> Self {
        EmptyJob { }
    }
}

impl Runnable for EmptyJob {
    fn run(&self) -> Result<RunSuccess, RunError> {
        Ok(RunSuccess::new(String::from("Successfully did nothing")))
    }
}
