extern crate serde;

// The macro is only used in the tests, but I can't remove it for normal build
#[allow(unused)]
#[macro_use]
extern crate serde_json;
extern crate rand;
extern crate notify_rust;

mod json_file;
mod from_value;
mod config;
mod runnable;
mod thread;
mod dispatcher;
mod job;

pub use job::Job;
pub use dispatcher::Dispatcher;

use json_file::JsonFile;
use from_value::FromValue;
use std::thread::park;
use std::env::home_dir;

fn main() {
    let mut path = home_dir().unwrap();

    path.push(".tasker/config.json");

    let mut config_file = JsonFile::new(path);

    config_file.open().unwrap();

    let json = config_file.json().to_owned();

    let dispatcher = Dispatcher::new_from_value(json).unwrap();

    dispatcher.dispatch();

    park();
}
