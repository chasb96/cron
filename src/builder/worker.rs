use builder::Builds;
use spawn::{Spawn, Spawns};
use timer::{Timer, Times};
use tokio::runtime::Runtime;

/// Struct to contain any object that can be run
///
/// Contains a Timer specifying when to be run,
///   and the Spawn to be run
#[derive(Debug, PartialEq, Deserialize)]
pub struct Worker {
    #[serde(flatten)]
    pub timer: Timer,

    #[serde(flatten)]
    pub spawn: Spawn,
}

impl Builds for Worker {
    fn build(self, runtime: &mut Runtime) {
        let spawn = self.spawn.clone();

        self.timer.time(runtime, move || spawn.spawn());
    }
}

impl Default for Worker {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
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
            timer: Timer::Interval(Interval::default()),
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
            timer: Timer::Interval(Interval::default()),
            spawn: Spawn::default(),
        };

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_call() {
        let mut runtime = Runtime::new().unwrap();

        let worker = Worker::default();

        worker.build(&mut runtime);
    }
}
