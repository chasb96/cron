use std::error::Error;
use timer::Times;
use timer::interval::Interval;
use timer::once::Once;
use tokio::runtime::Runtime;

/// `Enum` wrapping all the `Times`s together.
///
/// This derives interior `Times`s.
/// We go down this list until we find one we can derive. We start with the most amount of information given.
/// This behavior allows it to "fall" down the options until it finds a `Times` that works.
///
/// All `Times`s *must* impl the `Times` trait.
///
/// Derive precedence is as follows:
///   * Interval
///   * Once
#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Timer {
    Interval(Interval),
    Once(Once),
}

impl Times for Timer {
    /// Call dependent on `Timer`
    fn time<F>(&self, runtime: &mut Runtime, closure: F)
    where
        F: Fn() -> Result<(), Box<Error>> + Send + 'static,
    {
        match self {
            Timer::Interval(interval) => interval.time(runtime, closure),
            Timer::Once(once) => once.time(runtime, closure),
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer::Once(Once::default())
    }
}

impl Clone for Timer {
    fn clone(&self) -> Self {
        match self {
            Timer::Interval(interval) => Timer::Interval(interval.clone()),
            Timer::Once(once) => Timer::Once(once.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use timer::Times;

    #[test]
    fn test_once_with_delay() {
        let derived: Timer = serde_json::from_str(
            r#"{
                "delay": 0
            }"#,
        ).unwrap();

        let actual = Timer::Once(Once::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_once_without_delay() {
        let derived: Timer = serde_json::from_str(r#"{}"#).unwrap();

        let actual = Timer::Once(Once::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_interval_with_delay() {
        let derived: Timer = serde_json::from_str(
            r#"{
                "delay": 0,
                "interval": 1000
            }"#,
        ).unwrap();

        let actual = Timer::Interval(Interval::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_interval_without_delay() {
        let derived: Timer = serde_json::from_str(
            r#"{
                "interval": 1000
            }"#,
        ).unwrap();

        let actual = Timer::Interval(Interval::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_call() {
        let mut runtime = Runtime::new().unwrap();

        let derived: Timer = Timer::default();

        derived.time(&mut runtime, || Ok(()));
    }
}
