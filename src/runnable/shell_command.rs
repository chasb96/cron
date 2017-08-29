use std::process::Command;
use ::from_value::FromValue;
use serde_json::Value;
use ::runnable::{ Runnable, RunSuccess, RunError };

use std::error::Error;

#[derive(PartialEq, Debug, Eq)]
pub struct ShellCommand {
    command: String,
    args: Vec<String>,
}

impl ShellCommand {
    pub fn new(command: String, args: Vec<String>) -> Self {
        ShellCommand {
            command: command,
            args: args,
        }
    }
}

impl FromValue for ShellCommand {
    fn new_from_value(value: Value) -> Result<Self, Box<Error>> {
        let args = value.get("args")
                        .unwrap_or(&Value::Null)
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .to_owned();

        let args_as_stirng = args.into_iter().map( | arg | {
            String::from(arg.as_str().unwrap_or(""))
        }).collect();

        Ok(ShellCommand {
            command: String::from(value.get("command").unwrap_or(&Value::Null).as_str().unwrap_or("")),
            args: args_as_stirng,
        })
    }
}

impl Runnable for ShellCommand {
    fn run(&self) -> Result<RunSuccess, RunError> {
        match Command::new(&self.command)
            .args(&self.args)
            .spawn() {
                Ok(_) => Ok(RunSuccess::new(String::from("Notification thrown"))),
                Err(_) => Err(RunError::new(String::from("Notification failed to throw"))),
            }
    }
}

unsafe impl Send for ShellCommand { }

#[cfg(test)]
mod tests {
    use ::from_value::FromValue;
    use ::runnable::shell_command::ShellCommand;

    #[test]
    fn test_from_value() {
        let shell_command = json!({});

        ShellCommand::new_from_value(shell_command).unwrap();
    }
}
