use spawn::Spawns;
use spawn::command::Command;
use std::error::Error;

/// `Enum` wrapping all `Spawn`s together.
///
/// We use this to derive all `Spawn`s and their order.
/// When deriving, we "fall" down this list until we can find a `Spawn` that matches.
///
/// Derive precedence is as follows:
///   * Command
#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Spawn {
    Command(Command),
}

impl Spawns for Spawn {
    /// Create a default `Spawn`
    fn default() -> Self {
        Spawn::Command(Command::default())
    }

    /// Call the `Spawn`
    fn call(&self) -> Result<(), Box<Error>> {
        // All vartiants impl `Spawns`, we just map each variant to `.call()`.
        match self {
            Spawn::Command(command) => command.call(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
        let derived: Spawn = Spawn::default();

        derived.call().unwrap();
    }
}
