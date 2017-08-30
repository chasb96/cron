use notify::{ RecommendedWatcher, Watcher, RecursiveMode };
use std::sync::mpsc::channel;
use std::time::Duration;
use ::job::JobOption;

#[derive(Debug, PartialEq, Eq)]
pub struct File {
    path: String,
    job: JobOption,
}

impl File {
    pub fn new(path: String, job: JobOption) -> Self {
        File {
            path: path,
            job: job,
        }
    }
}

use ::runnable::{ Runnable, RunSuccess, RunError };

impl Runnable for File {
    fn run(&self) -> Result<RunSuccess, RunError> {
        let (rx, tx) = channel();

        let mut watcher: RecommendedWatcher = match Watcher::new(rx, Duration::from_millis(10)) {
            Ok(w) => w,
            Err(_) => return Err(RunError::new(String::from("Failed to create file watcher"))),
        };

        match watcher.watch(&self.path, RecursiveMode::Recursive) {
            Ok(w) => w,
            Err(_) => return Err(RunError::new(String::from("Failed to watch file"))),
        };

        loop {
            match tx.recv() {
                Ok(_) => self.job.run().unwrap(),
                Err(_) => return Err(RunError::new(String::from("Failed to spawn response job"))),
            };
        }
    }
}

use ::from_value::FromValue;
use serde_json::Value;
use std::error::Error;

impl FromValue for File {
    fn new_from_value(value: Value) -> Result<Self, Box<Error>> {
        Ok(
            File {
                path: String::new_from_value(value.get("path").unwrap().to_owned()).unwrap(),
                job: JobOption::new_from_value(value.get("job").unwrap_or(&json!({ "type": "empty" })).to_owned()).unwrap(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::File;
    use ::from_value::FromValue;

    #[test]
    fn test_from_value() {
        let listener = json!({ "path": "testing" });

        File::new_from_value(listener).unwrap();
    }
}
