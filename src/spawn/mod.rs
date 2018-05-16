pub use self::spawn::Spawn;

mod command;
mod spawn;

use std::error::Error;

pub trait Spawns {
    fn default() -> Self;

    fn call(&self) -> Result<(), Box<Error>>;
}
