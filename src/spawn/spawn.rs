use spawn::Spawns;
use spawn::command::Command;
use std::error::Error;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Spawn {
    Command(Command),
}

impl Spawns for Spawn {
    fn default() -> Self {
        Spawn::Command(Command::default())
    }

    fn call(&self) -> Result<(), Box<Error>> {
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
