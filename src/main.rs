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
    // let summary = String::from("Test");
    // let body = String::from("Testing 123");
    // let icon = String::from("");
    // let timeout = 0;
    //
    // let notification = Notification::new(summary, body, icon, timeout);
    //
    // // let notification_list = NotificationList::new(vec![notification]);
    //
    //
    // let summary_2 = String::from("Example");
    // let body_2 = String::from("Testing Second");
    // let icon_2 = String::from("");
    //
    // let notification_2 = Notification::new(summary_2, body_2, icon_2, timeout);
    //
    // let empty_job = EmptyJob::new();
    // let job = Job::new(empty_job, 1000);
    // let notification_list_2 = NotificationList::new(vec![notification_2, notification]);
    //
    // let job_2 = Job::new(notification_list_2, 1000);
    //
    //
    // let jobs = Vec::new();
    // jobs.push(job);
    // jobs.push(job_2);
    //
    // // job.spawn();
    // let dispatcher = Dispatcher::new(vec![job_2, job]);
    //
    // dispatcher.dispatch();
    let mut config_file = JsonFile::new(PathBuf::from("/home/charles/.cron/config.json"));

    config_file.open().unwrap();

    let json = config_file.json().to_owned();

    let dispatcher = Dispatcher::new_from_value(json);

    dispatcher.dispatch();

    park();
}
