use builder::Builds;
use builder::worker::Worker;
use std::error::Error;
use tokio::runtime::Runtime;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Builder {
    #[serde(default)]
    workers: Vec<Worker>,
}

impl Builds for Builder {
    fn build(self, runtime: &mut Runtime) -> Result<(), Box<Error>> {
        for worker in self.workers {
            worker.build(runtime)?;
        }

        Ok(())
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            workers: Vec::new(),
        }
    }
}

impl Clone for Builder {
    fn clone(&self) -> Self {
        Self {
            workers: self.workers.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_empty() {
        let derived: Builder = serde_json::from_str(r#"{}"#).unwrap();

        let actual = Builder::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_default() {
        let derived: Builder = serde_json::from_str(
            r#"{
                "workers": []
            }"#,
        ).unwrap();

        let actual = Builder::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_call() {
        let mut runtime = Runtime::new().unwrap();

        let builder = Builder::default();

        assert!(builder.build(&mut runtime).is_ok());
    }
}
