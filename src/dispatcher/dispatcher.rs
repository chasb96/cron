use ::Job;

use serde_json::Value;

use ::thread::Threadable;

pub struct Dispatcher {
    jobs: Vec<Job>,
}

impl Dispatcher {
    pub fn new(jobs: Vec<Job>) -> Self {
        Dispatcher {
            jobs: jobs
        }
    }

    pub fn dispatch(self) {
        for job in self.jobs {
            job.spawn();
        }
    }
}

use ::from_value::FromValue;

impl FromValue for Dispatcher {
    fn new_from_value(value: Value) -> Self {
        let jobs = value.get("jobs")
                        .unwrap_or(&Value::Null)
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .into_iter()
                        .map(| job | {
                            Job::new_from_value(job.to_owned())
                        }).collect();

        Dispatcher {
            jobs: jobs,
        }
    }
}
