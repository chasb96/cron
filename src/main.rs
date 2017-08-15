extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate notify_rust;

mod json_file;
mod from_value;
mod config;
mod notification;
mod run;
mod thread;
mod job;

use notification::Notification;
use job::Job;
use thread::Threadable;
use std::thread::park;

fn main() {
    let summary = String::from("Test");
    let body = String::from("Testing 123");
    let icon = String::from("");
    let timeout = 0;

    let notification = Notification::new(summary, body, icon, timeout);

    let job = Job::new(notification, 10);

    job.spawn();

    park();
}
