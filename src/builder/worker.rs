use builder::Builds;
use spawn::{Spawn, Spawns};
use timer::{Timer, Times};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Worker {
    #[serde(flatten)]
    when: Timer,

    #[serde(flatten)]
    what: Spawn,
}

impl Builds for Worker {
    fn default() -> Self {
        Self {
            when: Timer::default(),
            what: Spawn::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use timer::interval::Interval;

    #[test]
    fn test_once_with_delay() {
        let derived: Worker = serde_json::from_str(
            r#"{
                "delay": 0,
                "command": "echo"
            }"#,
        ).unwrap();

        let actual = Worker::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_once_without_delay() {
        let derived: Worker = serde_json::from_str(
            r#"{
                "command": "echo"
            }"#,
        ).unwrap();

        let actual = Worker::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_interval_with_delay() {
        let derived: Worker = serde_json::from_str(
            r#"{
                "delay": 0,
                "interval": 1000,
                "command": "echo"
            }"#,
        ).unwrap();

        let actual = Worker {
            when: Timer::Interval(Interval::default()),
            what: Spawn::default(),
        };

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_interval_without_delay() {
        let derived: Worker = serde_json::from_str(
            r#"{
                "interval": 1000,
                "command": "echo"
            }"#,
        ).unwrap();

        let actual = Worker {
            when: Timer::Interval(Interval::default()),
            what: Spawn::default(),
        };

        assert_eq!(derived, actual);
    }
}
