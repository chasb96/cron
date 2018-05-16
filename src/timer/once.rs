use timer::Timer;

/// Struct to allow a `Timer` dependent to run once and only once.
///
/// `Once` should be derived when:
///   * No specifications are given
///   * When only a delay is given
///   * Fallback when no other `Timer`s can be derived
#[derive(Debug, PartialEq, Deserialize)]
pub struct Once {
    /// How long to wait before running in `ms`.
    /// Defaults to `0`.
    #[serde(default)]
    delay: u32,
}

impl Timer for Once {
    /// Create a default `Once` `Timer`.
    ///
    /// ```
    /// Once {
    ///     delay: 0
    /// }
    /// ```
    fn default() -> Self {
        Once { delay: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn with_delay() {
        let derived: Once = serde_json::from_str(
            r#"{
                "delay": 0
            }"#,
        ).unwrap();

        let actual = Once::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn without_delay() {
        let derived: Once = serde_json::from_str(r#"{}"#).unwrap();

        let actual = Once::default();

        assert_eq!(derived, actual);
    }
}
