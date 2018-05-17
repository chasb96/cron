use std::time::{Duration, Instant};
use timer::Times;
use tokio;
use tokio::prelude::*;
use tokio::timer::{Delay, Error};

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

impl Times for Once {
    /// Create a default `Once` `Times`.
    ///
    /// ```
    /// Once {
    ///     delay: 0
    /// }
    /// ```
    fn default() -> Self {
        Once { delay: 0 }
    }

    /// Call the dependent on `Once`.
    fn call<F: Fn() -> Result<(), Error> + Send + 'static>(&self, f: Box<F>) {
        let time = Instant::now() + Duration::from_millis(self.delay);

        let call = Delay::new(time)
            .and_then(move |_| f())
            .map_err(|e| panic!("Failed to keep `Once` instance: {}", e));

        tokio::run(call);
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
        let once = Once::default();

        once.call(Box::new(|| Ok(())));
    }
}
