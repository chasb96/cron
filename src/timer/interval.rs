use std::time::{Duration, Instant};
use timer::Timer;
use tokio;
use tokio::prelude::*;
use tokio::timer::{Error, Interval as TokioInterval};

/// Struct to allow a `Timer` dependent to run repeatedly on an interval.
///
/// `Interval` should be derived when:
///   * An interval is given
#[derive(Debug, PartialEq, Deserialize)]
pub struct Interval {
    /// How long to wait before running in `ms`.
    /// Defaults to `0`.
    #[serde(default)]
    delay: u64,

    /// The wait time between executions in `ms`.
    /// Defaults to `1000`, or 1 second.
    interval: u64,
}

impl Timer for Interval {
    /// Create a default `Interval` `Timer`.
    ///
    /// ```
    /// Interval {
    ///     delay: 0,
    ///     interval: 1000
    /// }
    /// ```
    fn default() -> Self {
        Self {
            delay: 0,
            interval: 1000,
        }
    }

    /// Call the dependent on `Timer`.
    ///
    /// Returns the `Result` of the dependent.
    fn call<F: Fn() -> Result<(), Error> + Send + 'static>(&self, f: Box<F>) {
        let time = Instant::now() + Duration::from_millis(self.delay);

        let call = TokioInterval::new(time, Duration::from_millis(self.interval))
            .for_each(move |_| f())
            .map_err(|e| panic!("Failed to boot `Interval` instance: {}", e));

        tokio::run(call);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn with_delay() {
        let derived: Interval = serde_json::from_str(
            r#"{
                "delay": 0,
                "interval": 1000
            }"#,
        ).unwrap();

        let actual = Interval::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn without_delay() {
        let derived: Interval = serde_json::from_str(
            r#"{
                "interval": 1000
            }"#,
        ).unwrap();

        let actual = Interval::default();

        assert_eq!(derived, actual);
    }
}
