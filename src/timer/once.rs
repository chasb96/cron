use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::time::{Duration, Instant};
use timer::Times;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::timer::Delay;

/// Struct to allow a `Times` dependent to run once and only once.
///
/// `Once` should be derived when:
///   * No specifications are given
///   * When only a delay is given
///   * Fallback when no other `Times`s can be derived
#[derive(Debug, PartialEq, Deserialize)]
pub struct Once {
    /// How long to wait before running in `ms`.
    /// Defaults to `0`.
    #[serde(default)]
    delay: u64,
}

impl Once {
    pub fn delay(&self) -> Instant {
        Instant::now() + Duration::from_millis(self.delay)
    }
}

#[derive(Debug)]
struct OnceError;

impl Display for OnceError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Failed to keep `Once` instance")
    }
}

impl Error for OnceError {
    fn description(&self) -> &str {
        "Failed to keep `Once` instance"
    }
}

impl Times for Once {
    /// Call the dependent on `Once`.
    fn time<F>(&self, runtime: &mut Runtime, f: F)
    where
        F: FnOnce() -> Result<(), Box<Error>> + Send + 'static,
    {
        let call = Delay::new(self.delay())
            .and_then(|_| {
                f().unwrap();
                Ok(())
            })
            .map_err(|e| panic!("{}: {}", OnceError.description(), e));

        runtime.spawn(call);
    }
}

impl Default for Once {
    fn default() -> Self {
        Once { delay: 0 }
    }
}

impl Clone for Once {
    fn clone(&self) -> Self {
        Self { delay: self.delay }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_with_delay() {
        let derived: Once = serde_json::from_str(
            r#"{
                "delay": 0
            }"#,
        ).unwrap();

        let actual = Once::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_without_delay() {
        let derived: Once = serde_json::from_str(r#"{}"#).unwrap();

        let actual = Once::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_call() {
        let mut runtime = Runtime::new().unwrap();

        let once = Once::default();

        once.time(&mut runtime, || Ok(()));
    }
}
