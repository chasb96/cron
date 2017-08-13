use std::error::Error;

use ::list::List;

use serde_json::Value;

use rand::{ thread_rng, Rng };

pub struct RandomizedList {
    list: List,
}

impl RandomizedList {
    pub fn new(source: String) -> Self {
        RandomizedList {
            list: List::new(source),
        }
    }

    pub fn fill(&mut self) -> Result<&Vec<Value>, Box<Error>> {
        self.list.fill()
    }

    pub fn get(&self) -> Option<&Value> {
        let index: usize = thread_rng().gen_range(0, self.len());

        self.list.get(index)
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn set_list(&mut self, list: List) {
        self.list = list
    }

    pub fn list(&self) -> &List {
        &self.list
    }
}
