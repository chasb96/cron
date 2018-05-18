pub use self::config::Config;

mod builder;
mod config;
mod worker;

use std::error::Error;
use tokio::runtime::Runtime;

/// Trait to state an object can be built and run
pub trait Builds {
    fn build(self, runtime: &mut Runtime) -> Result<(), Box<Error>>;
}
