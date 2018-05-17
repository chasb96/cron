use std::time::{Duration, Instant};
use timer::Times;
use tokio;
use tokio::prelude::*;
use tokio::timer::{Error, Interval as TokioInterval};

/// Struct to allow a `Times` dependent to run repeatedly on an interval.
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

impl Times for Interval {
    /// Create a default `Interval` `Times`.
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

    /// Call the dependent on `Times`.
    fn call<F: Fn() -> Result<(), Error> + Send + 'static>(&self, f: Box<F>) {
        let time = Instant::now() + Duration::from_millis(self.delay);

        let call = TokioInterval::new(time, Duration::from_millis(self.interval))
            .for_each(move |_| f())
            .map_err(|e| panic!("Failed to keep `Interval` instance: {}", e));

        tokio::run(call);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use tokio::timer::Error;

    #[test]
    fn test_with_delay() {
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
    fn test_without_delay() {
        let derived: Interval = serde_json::from_str(
            r#"{
                "interval": 1000
            }"#,
        ).unwrap();

        let actual = Interval::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_call() {
        let interval = Interval::default();

        interval.call(Box::new(|| Err(Error::shutdown())));
    }
}
