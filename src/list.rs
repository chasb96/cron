use serde_json::Value;

use ::json_file::JsonFile;

use std::fmt;
use std::error::Error;

pub struct List {
    source: JsonFile,
    list: Vec<Value>,
}

impl List {
    pub fn new(source: String) -> Self {
        List {
            source: JsonFile::new(source),
            list: Vec::new(),
        }
    }

    pub fn fill(&mut self) -> Result<&Vec<Value>, Box<Error>> {
        let json = self.source.open()?;

        match json.json().as_array() {
            Some(list) => Ok(list),
            None => Err(Box::new(ListError)),
        }
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        if self.list.len() >= index && index >= 0 {
            Some(&self.list[index])
        }
        else {
            None
        }
     }

     pub fn len(&self) -> usize {
         self.list.len()
     }

     pub fn set_list(&mut self, list: Vec<Value>) {
         self.list = list
     }

     pub fn list(&self) -> &Vec<Value> {
         &self.list
     }

     pub fn set_source(&mut self, source: JsonFile) {
         self.source = source
     }

     pub fn source(&self) -> &JsonFile {
         &self.source
     }
}

#[derive(Debug)]
pub struct ListError;

impl fmt::Display for ListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JSON was not in form of array")
    }
}

impl Error for ListError {
    fn description(&self) -> &str {
        "JSON was not in form of array"
    }
}
