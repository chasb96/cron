use super::Waits;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::timer::Delay;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Once {
    #[serde(default)]
    delay: u64,
}

impl Waits for Once {
    fn wait<F>(&self, runtime: &mut Runtime, closure: F)
    where
        F: FnOnce() -> Result<(), Box<Error>> + Send + 'static,
    {
        let delay = Instant::now() + Duration::from_millis(self.delay);

        let future = Delay::new(delay)
            .and_then(|_| match closure() {
                // If the command given fails, I don't want it to crash;
                //     if it fails, the program called should handle its own
                _ => Ok(()),
            }).map_err(|e| panic!("{}: {}", OnceError.description(), e));

        runtime.spawn(future);
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

        once.wait(&mut runtime, || Ok(()));
    }
}
