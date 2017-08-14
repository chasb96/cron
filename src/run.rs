pub trait Runnable {
    fn run(&self) -> Result<RunSuccess, RunError>;
}

#[derive(Debug)]
pub struct RunSuccess {
    msg: String,
}

impl RunSuccess {
    pub fn new(msg: String) -> Self {
        RunSuccess {
            msg: msg,
        }
    }
}

#[derive(Debug)]
pub struct RunError {
    msg: String,
}

impl RunError {
    pub fn new(msg: String) -> Self {
        RunError {
            msg: msg,
        }
    }
}
