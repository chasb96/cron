use super::jobs::notification::Notification;
use super::jobs::notification_list::NotificationList;
use super::jobs::shell_command::ShellCommand;

use ::run::{ Runnable, RunSuccess, RunError };

use serde_json::Value;

pub enum JobOption {
    Empty,
    Notification(Notification),
    NotificationList(NotificationList),
    ShellCommand(ShellCommand),
}

impl JobOption {
    pub fn run(&self) -> Result<RunSuccess, RunError> {
        match self {
            &JobOption::Notification(ref n) => n.run(),
            &JobOption::NotificationList(ref nl) => nl.run(),
            &JobOption::ShellCommand(ref sc) => sc.run(),
            _ => Ok(RunSuccess::new(String::from("No job was specified"))),
        }
    }
}

use ::from_value::FromValue;

impl FromValue for JobOption {
    fn new_from_value(value: Value) -> Self {
        // TODO: make this less gross
        match value.get("type").unwrap_or(&Value::Null).as_str().unwrap_or("") {
            "notification" => JobOption::Notification(Notification::new_from_value(value.get("notification").unwrap_or(&Value::Null).to_owned())),
            "notification_list" => JobOption::NotificationList(NotificationList::new_from_value(value.get("notification_list").unwrap_or(&Value::Null).to_owned())),
            "shell_command" => JobOption::ShellCommand(ShellCommand::new_from_value(value.get("shell_command").unwrap_or(&Value::Null).to_owned())),
            _ => JobOption::Empty,
        }
    }
}

unsafe impl Send for JobOption { }
