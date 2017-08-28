use serde_json::Value;
use serde_json::value::Index;

use ::from_value::FromValue;
use ::json_file::{ JsonFile, FromJsonFile };

#[allow(dead_code)]
pub struct Config {
    config: Value,
}

#[allow(dead_code)]
impl Config {
    pub fn new(config: Value) -> Self {
        Config {
            config: config,
        }
    }

    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        self.config.get(index)
    }

    pub fn as_value(self) -> Value {
        self.config
    }
}

use std::error::Error;

impl FromValue for Config {
    fn new_from_value(config: Value) -> Result<Self, Box<Error>> {
        Ok(Config {
            config: config,
        })
    }
}

impl FromJsonFile for Config {
    fn new_from_json_file(file: JsonFile) -> Self {
        Config {
            config: file.json().to_owned()
        }
    }
}
