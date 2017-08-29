use ::thread::Threadable;
use ::from_value::FromValue;

use std::{ thread, time };

use super::job_option::JobOption;
use serde_json::Value;

pub struct Job {
    job: JobOption,
    interval: u64,
}

impl Job {
    pub fn new(job: JobOption, interval: u64) -> Self {
        Job {
            job: job,
            interval: interval
        }
    }
}

impl Threadable for Job {
    fn spawn(self) {
        thread::spawn(move | | {
            let sleep = time::Duration::from_millis(self.interval);

            loop {
                // TODO: Log this
                self.job.run().unwrap();

                thread::sleep(sleep);
            }
        });
    }
}

use std::error::Error;

impl FromValue for Job {
    fn new_from_value(value: Value) -> Result<Self, Box<Error>> {
        Ok(Job {
            interval: value.get("interval").unwrap_or(&Value::Null).as_u64().unwrap_or(1000),
            job:  JobOption::new_from_value(value)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use ::from_value::FromValue;
    use ::job::Job;

    #[test]
    fn test_from_value() {
        let value = json!({ "type": "empty" });

        Job::new_from_value(value).unwrap_or_else( | _ | panic!("Could not create job"));
    }
}
