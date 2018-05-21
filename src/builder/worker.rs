use builder::Builds;
use listener::{Listener, Waits};
use spawn::{Spawn, Spawns};
use std::error::Error;
use tokio::runtime::Runtime;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Worker {
    #[serde(flatten)]
    pub timer: Listener,

    #[serde(flatten)]
    pub spawn: Spawn,
}

impl Builds for Worker {
    fn build(self, runtime: &mut Runtime) -> Result<(), Box<Error>> {
        let spawn = self.spawn.clone();

        self.timer.wait(runtime, move || spawn.spawn());

        Ok(())
    }
}

impl Default for Worker {
    fn default() -> Self {
        Self {
            timer: Listener::default(),
            spawn: Spawn::default(),
        }
    }
}

impl Clone for Worker {
    fn clone(&self) -> Self {
        Self {
            timer: self.timer.clone(),
            spawn: self.spawn.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use listener::interval::Interval;
    use serde_json;

    #[test]
    fn test_empty() {
        let derived: Worker = serde_json::from_str(r#"{}"#).unwrap();

        let actual = Worker::default();

        assert_eq!(derived, actual);
    }

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
            timer: Listener::Interval(Interval::default()),
            spawn: Spawn::default(),
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
            timer: Listener::Interval(Interval::default()),
            spawn: Spawn::default(),
        };

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_call() {
        let mut runtime = Runtime::new().unwrap();

        let worker = Worker::default();

        assert!(worker.build(&mut runtime).is_ok());
    }
}
