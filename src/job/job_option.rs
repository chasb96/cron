use ::runnable::notification::Notification;
use ::runnable::notification_list::NotificationList;
use ::runnable::shell_command::ShellCommand;

use ::runnable::{ Runnable, RunSuccess, RunError };

use serde_json::Value;

#[derive(Debug, PartialEq, Eq)]
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
    fn new_from_value(value: Value) -> Result<Self, Box<Error>> {
        // TODO: make this less gross
        match value.get("type").unwrap_or(&Value::Null).as_str().unwrap_or("") {
            "notification" => Ok(JobOption::Notification(Notification::new_from_value(value.get("notification").unwrap_or(&Value::Null).to_owned()).unwrap())),
            "notification_list" => Ok(JobOption::NotificationList(NotificationList::new_from_value(value.get("notification_list").unwrap_or(&Value::Null).to_owned()).unwrap())),
            "shell_command" => Ok(JobOption::ShellCommand(ShellCommand::new_from_value(value.get("shell_command").unwrap_or(&Value::Null).to_owned()).unwrap())),
            _ => Err(Box::new(JobOptionError)),
        }
    }
}

unsafe impl Send for JobOption { }

use std::error::Error;
use std::fmt::{ Display, Formatter, Result as FmtResult };

#[derive(Debug)]
struct JobOptionError;

impl Display for JobOptionError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        "The selected option is not available".fmt(f)
    }
}

impl Error for JobOptionError {
    fn description(&self) -> &str {
        "An error occurred creating the Job"
    }
}

#[cfg(test)]
mod tests {
    use ::from_value::FromValue;
    use ::job::job_option::JobOption;

    #[test]
    fn test_notification() {
        let notification = json!({ "type": "notification", "notification": {}});

        match JobOption::new_from_value(notification) {
            Err(_) => panic!("Was not able to create notification"),
            Ok(_) => { }
        }
    }
}
