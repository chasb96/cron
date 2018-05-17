use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::time::{Duration, Instant};
use timer::Times;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::timer::Interval as TokioInterval;

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

#[derive(Debug)]
struct IntervalError;

impl Display for IntervalError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Failed to keep `Interval` instance")
    }
}

impl Error for IntervalError {
    fn description(&self) -> &str {
        "Failed to keep `Interval` instance"
    }
}

impl Interval {
    pub fn delay(&self) -> Instant {
        Instant::now() + Duration::from_millis(self.delay)
    }

    pub fn interval(&self) -> Duration {
        Duration::from_millis(self.interval)
    }
}

impl Times for Interval {
    /// Call the dependent on `Times`.
    fn time<F>(&self, runtime: &mut Runtime, f: F)
    where
        F: Fn() -> Result<(), Box<Error>> + Send + 'static,
    {
        let call = TokioInterval::new(self.delay(), self.interval())
            .for_each(move |_| {
                f().unwrap();
                Ok(())
            })
            .map_err(|e| println!("{}: {}", IntervalError.description(), e));

        runtime.spawn(call);
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            delay: 0,
            interval: 1000,
        }
    }
}

impl Clone for Interval {
    fn clone(&self) -> Self {
        Self {
            delay: self.delay,
            interval: self.interval,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
        let mut runtime = Runtime::new().unwrap();

        let interval = Interval::default();

        interval.time(&mut runtime, || Ok(()));
    }
}
