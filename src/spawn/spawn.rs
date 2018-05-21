use spawn::Spawns;
use spawn::command::Command;
use std::error::Error;

/// Every option in `Spawn` must impl the `Spawns` trait.
///   Similar to `Times`, we can't enforce this in the compiler.
///   However, without this, it's easy for the code to become unmanageable.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Spawn {
    Command(Command),
}

impl Spawns for Spawn {
    fn spawn(&self) -> Result<(), Box<Error>> {
        match self {
            Spawn::Command(command) => command.spawn(),
        }
    }
}

impl Default for Spawn {
    fn default() -> Self {
        Spawn::Command(Command::default())
    }
}

impl Clone for Spawn {
    fn clone(&self) -> Self {
        match self {
            Spawn::Command(command) => Spawn::Command(command.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_empty_with_args() {
        let derived: Spawn = serde_json::from_str(
            r#"{
                "args": []
            }"#,
        ).unwrap();

        let actual = Spawn::Command(Command::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_empty_without_args() {
        let derived: Spawn = serde_json::from_str(r#"{}"#).unwrap();

        let actual = Spawn::Command(Command::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_command_with_args() {
        let derived: Spawn = serde_json::from_str(
            r#"{
                "command": "echo",
                "args": []
            }"#,
        ).unwrap();

        let actual = Spawn::Command(Command::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_command_without_args() {
        let derived: Spawn = serde_json::from_str(
            r#"{
                "command": "echo"
            }"#,
        ).unwrap();

        let actual = Spawn::Command(Command::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn call() {
        let result = Spawn::default().spawn();

        assert!(result.is_ok())
    }
}
