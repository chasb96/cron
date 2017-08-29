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

    pub fn variant_eq(&self, variant: &str) -> bool {
        match (self, variant) {
            (&JobOption::Empty, "empty") => true,
            (&JobOption::Notification(_), "notification") => true,
            (&JobOption::NotificationList(_), "notification_list") => true,
            (&JobOption::ShellCommand(_), "shell_command") => true,
            _ => false,
        }
    }
}

use ::from_value::FromValue;

impl FromValue for JobOption {
    fn new_from_value(value: Value) -> Result<Self, Box<Error>> {
        // TODO: make this less gross
        match value.get("type").unwrap_or(&Value::Null).as_str().unwrap_or("") {
            "empty" => Ok(JobOption::Empty),
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
    use ::runnable::notification::Notification;
    use ::runnable::notification_list::NotificationList;
    use ::runnable::shell_command::ShellCommand;
    use serde_json::Value;

    #[test]
    fn test_variants() {
        assert!(JobOption::Empty.variant_eq("empty"));
        assert!(JobOption::Notification(Notification::new_from_value(Value::Null).unwrap()).variant_eq("notification"));
        assert!(JobOption::NotificationList(NotificationList::new_from_value(Value::Null).unwrap()).variant_eq("notification_list"));
        assert!(JobOption::ShellCommand(ShellCommand::new_from_value(Value::Null).unwrap()).variant_eq("shell_command"));
    }

    #[test]
    fn test_empty() {
        let empty = json!({ "type": "empty" });

        let option = JobOption::new_from_value(empty).unwrap();

        assert!(option.variant_eq("empty"));
    }

    #[test]
    fn test_notification() {
        let notification = json!({ "type": "notification", "notification": {}});

        let option = JobOption::new_from_value(notification).unwrap();

        assert!(option.variant_eq("notification"));
    }

    #[test]
    fn test_notification_list() {
        let notification_list = json!({ "type": "notification_list", "notification_list": {}});

        let option = JobOption::new_from_value(notification_list).unwrap();

        assert!(option.variant_eq("notification_list"));
    }

    #[test]
    fn test_shell_command() {
        let shell_command = json!({ "type": "shell_command", "shell_command": {}});

        let option = JobOption::new_from_value(shell_command).unwrap();

        assert!(option.variant_eq("shell_command"));
    }
}
