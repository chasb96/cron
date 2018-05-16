use timer::Timer;

/// Struct to allow a `Timer` dependent to run repeatedly on an interval.
///
/// `Interval` should be derived when:
///   * An interval is given
#[derive(Debug, PartialEq, Deserialize)]
pub struct Interval {
    /// How long to wait before running in `ms`.
    /// Defaults to `0`.
    #[serde(default)]
    delay: u32,

    /// The wait time between executions in `ms`.
    /// Defaults to `1000`, or 1 second.
    interval: u32,
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
