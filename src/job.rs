use ::run::Runnable;
use ::thread::Threadable;

use std::{ thread, time };
use std::sync::{ Arc, Mutex };

pub struct Job<R: Runnable + Send + 'static> {
    job: R,
    interval: u64,
}

impl<R: Runnable + Send + 'static> Job<R> {
    pub fn new(job: R, interval: u64) -> Self {
        Job {
            job: job,
            interval: interval
        }
    }
}

impl<R: Runnable + Send + 'static> Threadable for Job<R> {
    fn spawn(self) {
        thread::spawn(move | | {
            let sleep = time::Duration::from_millis(self.interval);

            loop {
                thread::sleep(sleep);

                self.job.run();
            }
        });
    }
}
