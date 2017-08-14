use serde_json;
use serde_json::Value;
use serde_json::value::Index;

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub struct JsonFile {
    path: PathBuf,
    json: Value,
}

impl JsonFile {
    pub fn new(path: PathBuf) -> Self {
        JsonFile {
            path: path,
            json: Value::Null,
        }
    }

    pub fn open(&mut self) -> Result<&Self, Box<Error>> {
        // Open file for reading
        let file = File::open(&self.path)?;

        // Read the file into JSON
        &self.set_json(serde_json::from_reader(file)?);

        Ok(self)
    }

    pub fn json(&self) -> &Value {
        &self.json
    }

    pub fn set_json(&mut self, json: Value) {
        self.json = json
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path
    }

    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        self.json().get(index)
    }
}

pub trait FromJsonFile {
    fn new_from_json_file(file: JsonFile) -> Self;
}
