use builder::Builds;
use builder::worker::Worker;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use tokio::runtime::Runtime;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Builder {
    workers: Vec<Worker>,
}

impl Builder {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<Error>> {
        let file = File::open(path)?;

        let builder = serde_json::from_reader(file)?;

        Ok(builder)
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

impl Builds for Builder {
    fn build(self, runtime: &mut Runtime) {
        for worker in self.workers {
            worker.build(runtime)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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

        builder.build(&mut runtime);
    }
}
