use default_path;
use listener::Waits;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::fs::Metadata;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::timer::Interval;

#[derive(Debug, PartialEq, Deserialize)]
pub struct File {
    #[serde(rename = "file")]
    path: PathBuf,

    #[serde(default = "default_interval")]
    interval: u64,
}

fn default_interval() -> u64 {
    1000
}

impl File {
    fn modified(&self) -> Result<bool, Box<Error>> {
        let metadata = self.metadata()?;

        let modified = metadata.modified()?;
        let now = SystemTime::now();
        let interval = Duration::from_millis(self.interval);

        match now.duration_since(modified)? < interval {
            true => Ok(true),
            false => Ok(false),
        }
    }

    fn metadata(&self) -> Result<Metadata, Box<Error>> {
        let file = fs::File::open(&self.path)?;
        let metadata = file.metadata()?;

        Ok(metadata)
    }
}

impl Waits for File {
    fn wait<F>(&self, runtime: &mut Runtime, closure: F)
    where
        F: Fn() -> Result<(), Box<Error>> + Send + 'static,
    {
        let now = Instant::now();

        let interval = Duration::from_millis(self.interval);

        let file = self.clone();

        let future = Interval::new(now, interval)
            .for_each(move |_| match file.modified() {
                Ok(true) => match closure() {
                    _ => Ok(()),
                },
                // If the command given fails, I don't want it to stop;
                //     if it fails, the program called should handle its own
                _ => Ok(()),
            })
            .map_err(|e| println!("{}: {}", FileError.description(), e));

        runtime.spawn(future);
    }
}

impl Default for File {
    fn default() -> Self {
        let default_path = default_path().unwrap_or(PathBuf::from("tasker.json"));

        Self {
            path: default_path,
            interval: 1000,
        }
    }
}

impl Clone for File {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            interval: self.interval,
        }
    }
}

#[derive(Debug)]
struct FileError;

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Failed to keep `File` instance")
    }
}

impl Error for FileError {
    fn description(&self) -> &str {
        "Failed to keep `File` instance"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn path_str() -> String {
        let default_path = default_path().unwrap();
        String::from(default_path.to_str().unwrap())
    }

    #[test]
    fn test_default() {
        let json = format!(
            "{{
                \"file\": \"{}\",
                \"interval\": 1000
            }}",
            path_str()
        );

        let derived: File = serde_json::from_str(&json).unwrap();

        let actual = File::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_without_interval() {
        let json = format!(
            "{{
                \"file\": \"{}\"
            }}",
            path_str()
        );

        let derived: File = serde_json::from_str(&json).unwrap();

        let actual = File::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_call() {
        let mut runtime = Runtime::new().unwrap();

        let once = File::default();

        once.wait(&mut runtime, || Ok(()));
    }
}
