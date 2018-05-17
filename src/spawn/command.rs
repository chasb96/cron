use spawn::Spawns;
use std::error::Error;
use std::process::Command as Cmd;

/// Struct to create a call to another executable.
///
/// `Command` should be derived when:
///   * A command is given
#[derive(Debug, PartialEq, Deserialize)]
pub struct Command {
    /// The executable to call.
    /// There's only a handful of commands on all systems:
    /// Defaults to `"echo"`
    command: String,

    /// The arguments to pass to the executable.
    /// Defaults to an empty set, `[]`
    #[serde(default)]
    args: Vec<String>,
}

impl Spawns for Command {
    /// Call the `Command`/Tell the `Command` to execute
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
