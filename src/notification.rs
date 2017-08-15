use notify_rust::{ Notification as NativeNotification, NotificationHandle, Error };

use ::from_value::FromValue;
use ::run::{ Runnable, RunSuccess, RunError };

use serde_json::{ Value, Number };

pub struct Notification {
    summary: String,
    body: String,
    icon: String,
    timeout: i32,
}

impl Notification {
    pub fn new(summary: String, body: String, icon: String, timeout: i32) -> Self {
        Notification {
            summary: summary,
            body: body,
            icon: icon,
            timeout: timeout,
        }
    }

    pub fn summary(&self) -> &String {
        &self.summary
    }

    pub fn set_summary(&mut self, summary: String) {
        self.summary = summary
    }

    pub fn body(&self) -> &String {
        &self.body
    }

    pub fn set_body(&mut self, body: String) {
        self.body = body
    }

    pub fn icon(&self) -> &String {
        &self.icon
    }

    pub fn set_icon(&mut self, icon: String) {
        self.icon = icon
    }

    pub fn timeout(&self) -> i32 {
        self.timeout
    }

    pub fn set_timeout(&mut self, timeout: i32) {
        self.timeout = timeout
    }
}

impl FromValue for Notification {
    fn new_from_value(value: Value) -> Self {
        Notification {
            summary: String::new_from_value(value.get("summary").unwrap_or(&Value::Null).to_owned()),
            body: String::new_from_value(value.get("body").unwrap_or(&Value::Null).to_owned()),
            icon: String::new_from_value(value.get("icon").unwrap_or(&Value::Null).to_owned()),
            timeout: i32::new_from_value(value.get("timeout").unwrap_or(&Value::Null).to_owned()),
        }
    }
}

impl Runnable for Notification {
    fn run(&self) -> Result<RunSuccess, RunError> {
        match NativeNotification::new()
            .summary(&self.summary)
            .body(&self.body)
            .icon(&self.icon)
            .timeout(self.timeout)
            .show() {
                Ok(_) => Ok(RunSuccess::new(String::from("Notification thrown"))),
                Err(_) => Err(RunError::new(String::from("Notification failed to throw"))),
            }
    }
}

unsafe impl Send for Notification {
    
}
