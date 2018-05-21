pub use self::spawn::Spawn;

pub mod command;
pub mod spawn;

use std::error::Error;

pub trait Spawns {
    fn spawn(&self) -> Result<(), Box<Error>>;
}
