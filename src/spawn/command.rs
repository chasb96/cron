use spawn::Spawns;
use std::error::Error;
use std::process::Command as Cmd;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Command {
    command: String,

    #[serde(default)]
    args: Vec<String>,
}

impl Spawns for Command {
    fn default() -> Self {
        Self {
            command: String::from("echo"),
            args: Vec::new(),
        }
    }

    fn call(&self) -> Result<(), Box<Error>> {
        Cmd::new(&self.command).args(&self.args).spawn()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
        let result = Command::default().call();

        assert!(result.is_ok())
    }
}
