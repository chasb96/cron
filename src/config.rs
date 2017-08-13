use serde_json::Value;
use serde_json::value::Index;

use ::json_file::JsonFile;

use std::error::Error;

pub struct Config {
    path: String,
    config: JsonFile,
}

impl Config {
    pub fn new(path: String) -> Self {
        Config {
            path: path.clone(),
            config: JsonFile::new(path),
        }
    }

    pub fn open(&mut self) -> Result<&Self, Box<Error>> {
        self.config.open()?;

        Ok(self)
    }

    pub fn config(&self) -> &JsonFile {
        &self.config
    }

    pub fn set_config(&mut self, config: JsonFile) {
        self.config = config
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path
    }

    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        self.config().get(index)
    }
}
