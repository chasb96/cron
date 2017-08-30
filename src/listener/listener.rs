use ::thread::Threadable;
use ::from_value::FromValue;

use std::thread;

use super::listener_option::ListenerOption;
use serde_json::Value;

// I know this is a little redundant,
// but I wanted to preserve the same structure from Jobs
// and if any separate fields are added, they can be added independently
pub struct Listener {
    listener: ListenerOption
}

impl Listener {
    pub fn new(listener: ListenerOption) -> Self {
        Listener {
            listener: listener,
        }
    }
}


impl Threadable for Listener {
    fn spawn(self) {
        thread::spawn(move | | {
            self.listener.run().unwrap();
        });
    }
}

use std::error::Error;

impl FromValue for Listener {
    fn new_from_value(value: Value) -> Result<Self, Box<Error>> {
        Ok(
            Listener {
                listener: ListenerOption::new_from_value(value)?
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use ::from_value::FromValue;
    use super::Listener;

    #[test]
    fn test_from_value() {
        let value = json!({ "type": "empty" });

        Listener::new_from_value(value).unwrap_or_else( | _ | panic!("Could not create listener"));
    }
}
