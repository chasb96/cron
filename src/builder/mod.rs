pub use self::config::Config;

mod builder;
mod config;
mod worker;

use std::error::Error;
use std::path::Path;
use tokio::runtime::Runtime;

/// We need the ability to clone for safety issues.
///   The items being built can be sent to asynchronous methods,
///   which can create a mountain of memory issues.
pub trait Builds: Clone {
    fn build(self, runtime: &mut Runtime) -> Result<(), Box<Error>>;
}

/// Default: without this, what we're building cloudn't be derived completely.
/// Sized: needed in Result.
pub trait FromFile: Default + Sized {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<Error>>;
}
