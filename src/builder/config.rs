use builder::Builds;
use builder::builder::Builder;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use tokio::runtime::Runtime;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    #[serde(default)]
    configs: Vec<String>,

    #[serde(default, flatten)]
    builder: Builder,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<Error>> {
        let file = File::open(path)?;

        let builder = serde_json::from_reader(file)?;

        Ok(builder)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            configs: Vec::new(),
            builder: Builder::default(),
        }
    }
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Self {
            configs: self.configs.clone(),
            builder: self.builder.clone(),
        }
    }
}

impl Builds for Config {
    fn build(self, runtime: &mut Runtime) -> Result<(), Box<Error>> {
        for config in self.configs {
            Config::from_file(config)?.build(runtime)?;
        }

        self.builder.build(runtime)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_empty() {
        let derived: Config = serde_json::from_str(r#"{}"#).unwrap();

        let actual = Config::default();

        assert_eq!(derived, actual);
    }

    #[test]
    fn test_with_builder() {
        let derived: Config = serde_json::from_str(
            r#"{
                "workers": []
            }"#,
        ).unwrap();

        let actual = Config::default();

        assert_eq!(derived, actual);
    }
}
