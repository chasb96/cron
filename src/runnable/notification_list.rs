extern crate rand;

use ::from_value::FromValue;
use ::runnable::{ Runnable, RunSuccess, RunError };
use super::notification::Notification;

use rand::Rng;
use serde_json::Value;

#[derive(Debug, PartialEq, Eq)]
pub struct NotificationList {
    items: Vec<Notification>,
}

impl NotificationList {
    pub fn new(items: Vec<Notification>) -> Self {
        NotificationList {
            items: items
        }
    }

    pub fn items(&self) -> &Vec<Notification> {
        &self.items
    }

    pub fn set_items(&mut self, items: Vec<Notification>) {
        self.items = items
    }
}

impl Runnable for NotificationList {
    fn run(&self) -> Result<RunSuccess, RunError> {
        match rand::thread_rng().choose(self.items()) {
            Some(notification) => {
                notification.run()?;

                Ok(RunSuccess::new(String::from("Ran random notification from list")))
            },
            None => {
                Err(RunError::new(String::from("Could not find notification to run")))
            }
        }
    }
}

use std::error::Error;

impl FromValue for NotificationList {
    fn new_from_value(value: Value) -> Result<Self, Box<Error>> {
        let list = value.as_array().unwrap_or(&Vec::new()).to_owned();

        let notification_list: Vec<Notification> = list.into_iter().map(| v | {
            Notification::new_from_value(v.to_owned()).unwrap()
        }).collect();

        Ok(NotificationList {
            items: notification_list
        })
    }
}

unsafe impl Send for NotificationList { }
