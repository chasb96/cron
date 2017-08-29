use super::listeners::File;
use ::runnable::{ Runnable, RunSuccess, RunError };

use serde_json::Value;

#[derive(Debug, PartialEq, Eq)]
pub enum ListenerOption {
    Empty,
    File(File),
}

impl ListenerOption {
    pub fn run(&self) -> Result<RunSuccess, RunError> {
        match self {
            &ListenerOption::File(ref n) => n.run(),
            _ => Ok(RunSuccess::new(String::from("No listener was specified"))),
        }
    }

    pub fn variant_eq(&self, variant: &str) -> bool {
        match (self, variant) {
            (&ListenerOption::Empty, "empty") => true,
            (&ListenerOption::File(_), "file") => true,
            _ => false,
        }
    }
}

use ::from_value::FromValue;

impl FromValue for ListenerOption {
    fn new_from_value(value: Value) -> Result<Self, Box<Error>> {
        // TODO: make this less gross
        match value.get("type").unwrap_or(&Value::Null).as_str().unwrap_or("") {
            "empty" => Ok(ListenerOption::Empty),
            "file" => Ok(ListenerOption::File(File::new_from_value(value.get("file").unwrap_or(&Value::Null).to_owned()).unwrap())),
            _ => Err(Box::new(ListenerOptionError)),
        }
    }
}

unsafe impl Send for ListenerOption { }

use std::error::Error;
use std::fmt::{ Display, Formatter, Result as FmtResult };

#[derive(Debug)]
struct ListenerOptionError;

impl Display for ListenerOptionError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        "The selected option is not available".fmt(f)
    }
}

impl Error for ListenerOptionError {
    fn description(&self) -> &str {
        "An error occurred creating the Listener"
    }
}

#[cfg(test)]
mod tests {
    use ::from_value::FromValue;
    use super::ListenerOption;
    use ::listener::listeners::File;

    #[test]
    fn test_variants() {
        assert!(ListenerOption::Empty.variant_eq("empty"));
        assert!(ListenerOption::File(File::new_from_value(json!({ "path": "testing" })).unwrap()).variant_eq("file"));
    }

    #[test]
    fn test_empty() {
        let empty = json!({ "type": "empty" });

        let option = ListenerOption::new_from_value(empty).unwrap();

        assert!(option.variant_eq("empty"));
    }

    #[test]
    fn test_file() {
        let file = json!({ "type": "file", "file": { "path": "testing" } });

        let option = ListenerOption::new_from_value(file).unwrap();

        assert!(option.variant_eq("file"));
    }
}
