use super::Waits;
use super::interval::Interval;
use super::once::Once;
use std::error::Error;
use tokio::runtime::Runtime;

/// Every option in `Listener` must impl the `Times` trait...
///   Since the compiler cannot enforce this, it must be done by hand.
///   Without this, the code easily becomes smelly and unmanageable.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Listener {
    Interval(Interval),
    Once(Once),
}

impl Waits for Listener {
    fn wait<F>(&self, runtime: &mut Runtime, closure: F)
    where
        F: Fn() -> Result<(), Box<Error>> + Send + 'static,
    {
        match self {
            Listener::Interval(interval) => interval.wait(runtime, closure),
            Listener::Once(once) => once.wait(runtime, closure),
        }
    }
}

impl Default for Listener {
    fn default() -> Self {
        Listener::Once(Once::default())
    }
}

impl Clone for Listener {
    fn clone(&self) -> Self {
        match self {
            Listener::Interval(interval) => Listener::Interval(interval.clone()),
            Listener::Once(once) => Listener::Once(once.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_once_with_delay() {
        let derived: Listener = serde_json::from_str(
            r#"{
                "delay": 0
            }"#,
        ).unwrap();

        let actual = Listener::Once(Once::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_once_without_delay() {
        let derived: Listener = serde_json::from_str(r#"{}"#).unwrap();

        let actual = Listener::Once(Once::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_interval_with_delay() {
        let derived: Listener = serde_json::from_str(
            r#"{
                "delay": 0,
                "interval": 1000
            }"#,
        ).unwrap();

        let actual = Listener::Interval(Interval::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_interval_without_delay() {
        let derived: Listener = serde_json::from_str(
            r#"{
                "interval": 1000
            }"#,
        ).unwrap();

        let actual = Listener::Interval(Interval::default());

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_call() {
        let mut runtime = Runtime::new().unwrap();

        let derived: Listener = Listener::default();

        derived.wait(&mut runtime, || Ok(()));
    }
}
