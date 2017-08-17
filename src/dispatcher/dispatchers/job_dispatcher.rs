use ::Job;

pub struct JobDispatcher {
    jobs: Vec<Job>
}

impl JobDispatcher {
    pub fn new(jobs: Vec<Job>) -> Self {
        JobDispatcher {
            jobs: jobs
        }
    }
}

use ::from_value::FromValue;

impl FromValue for JobDispatcher {

}
