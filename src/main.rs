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

use json_file::JsonFile;
use dispatcher::Dispatcher;
use from_value::FromValue;
use std::thread::park;
use std::env::home_dir;

fn main() {
    let mut path = home_dir().unwrap();

    path.push(".cron/tasker.json");

    let mut config_file = JsonFile::new(path);

    config_file.open().unwrap();

    let json = config_file.json().to_owned();

    let dispatcher = Dispatcher::new_from_value(json);

    dispatcher.dispatch();

    park();
}
