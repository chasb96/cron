use timer::Timer;
use timer::interval::Interval;
use timer::once::Once;
use tokio::timer::Error;

/// `Enum` wrapping all the `Timer`s together.
///
/// This derives interior `Timer`s.
/// We go down this list until we find one we can derive. We start with the most amount of information given.
/// This behavior allows it to "fall" down the options until it finds a `Timer` that works.
///
/// All `Timer`s *must* impl the `Timer` trait.
///
/// Derive precedence is as follows:
///   * Interval
///   * Once
#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Timing {
    Interval(Interval),
    Once(Once),
}

impl Timer for Timing {
    fn default() -> Self {
        Timing::Once(Once::default())
    }

    fn call<F: Fn() -> Result<(), Error> + Send + 'static>(&self, f: Box<F>) {
        match self {
            Timing::Interval(interval) => interval.call(f),
            Timing::Once(once) => once.call(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use timer::Timer;

    #[test]
    fn once_with_delay() {
        let derived: Timing = serde_json::from_str(
            r#"{
                "delay": 0
            }"#,
        ).unwrap();

        let actual = Timing::Once(Once::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn once_without_delay() {
        let derived: Timing = serde_json::from_str(r#"{}"#).unwrap();

        let actual = Timing::Once(Once::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn interval_with_delay() {
        let derived: Timing = serde_json::from_str(
            r#"{
                "delay": 0,
                "interval": 1000
            }"#,
        ).unwrap();

        let actual = Timing::Interval(Interval::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn interval_without_delay() {
        let derived: Timing = serde_json::from_str(
            r#"{
                "interval": 1000
            }"#,
        ).unwrap();

        let actual = Timing::Interval(Interval::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn call() {
        let derived: Timing = Timing::default();

        derived.call(Box::new(|| Ok(())));
    }
}
