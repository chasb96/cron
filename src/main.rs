extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate notify_rust;

mod json_file;
mod from_value;
mod config;
mod run;
mod thread;
mod dispatcher;
mod job;

pub use job::Job;

use std::path::PathBuf;
use json_file::JsonFile;
use dispatcher::Dispatcher;
use from_value::FromValue;
use std::thread::park;

fn main() {
    let mut config_file = JsonFile::new(PathBuf::from("/home/charles/.cron/config.json"));

    config_file.open().unwrap();

    let json = config_file.json().to_owned();

    let dispatcher = Dispatcher::new_from_value(json);

    dispatcher.dispatch();

    park();
}
