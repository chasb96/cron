use super::Waits;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::timer::Interval as TokioInterval;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Interval {
    #[serde(default)]
    delay: u64,

    // Do NOT derive interval: we want to derive when this IS specified
    interval: u64,
}

impl Waits for Interval {
    fn wait<F>(&self, runtime: &mut Runtime, closure: F)
    where
        F: Fn() -> Result<(), Box<Error>> + Send + 'static,
    {
        let delay = Instant::now() + Duration::from_millis(self.delay);

        let interval = Duration::from_millis(self.interval);

        let future = TokioInterval::new(delay, interval)
            .for_each(move |_| match closure() {
                // If the command given fails, I don't want it to stop;
                //     if it fails, the program called should handle its own
                _ => Ok(()),
            })
            .map_err(|e| println!("{}: {}", IntervalError.description(), e));

        runtime.spawn(future);
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

        interval.wait(&mut runtime, || Ok(()));
    }
}
