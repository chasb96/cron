use spawn::Spawns;
use std::error::Error;
use std::process::Command as Cmd;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Command {
    #[serde(default = "default_command")]
    command: String,
    #[serde(default)]
    args: Vec<String>,
}

/// serde derives empty string for default of String.
fn default_command() -> String {
    "echo".to_string()
}

impl Spawns for Command {
    fn spawn(&self) -> Result<(), Box<Error>> {
        Cmd::new(&self.command).args(&self.args).spawn()?;

        Ok(())
    }
}

impl Default for Command {
    fn default() -> Self {
        Self {
            command: String::from("echo"),
            args: Vec::new(),
        }
    }
}

impl Clone for Command {
    fn clone(&self) -> Self {
        Self {
            command: self.command.clone(),
            args: self.args.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_empty_with_args() {
        let derived: Command = serde_json::from_str(
            r#"{
                "args": []
            }"#,
        ).unwrap();

        let actual = Command::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_empty_without_args() {
        let derived: Command = serde_json::from_str(r#"{}"#).unwrap();

        let actual = Command::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_with_args() {
        let derived: Command = serde_json::from_str(
            r#"{
                "command": "echo",
                "args": []
            }"#,
        ).unwrap();

        let actual = Command::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_without_args() {
        let derived: Command = serde_json::from_str(
            r#"{
                "command": "echo"
            }"#,
        ).unwrap();

        let actual = Command::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_call() {
        let result = Command::default().spawn();

        assert!(result.is_ok())
    }
}
