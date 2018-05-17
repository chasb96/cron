use timer::Times;
use timer::interval::Interval;
use timer::once::Once;
use tokio::timer::Error;

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
    /// Create a default `Times`
    fn default() -> Self {
        Timer::Once(Once::default())
    }

    /// Call dependent on `Timer`
    fn call<F: Fn() -> Result<(), Error> + Send + 'static>(&self, f: Box<F>) {
        // Just call the variant:
        // All `Timer` variants must impl `Times`, so we just use `.call(f)` on all variants
        match self {
            Timer::Interval(interval) => interval.call(f),
            Timer::Once(once) => once.call(f),
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
        let derived: Timer = Timer::default();

        derived.call(Box::new(|| Ok(())));
    }
}
