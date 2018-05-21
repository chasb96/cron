pub use self::spawn::Spawn;

pub mod command;
pub mod spawn;

use std::error::Error;

/// Spawns needs Default + Clone for UX and safety reasons:
///   Default to allow simple deriving for serde
///   Clone to allow safe usage of a Spawn asynchronously
pub trait Spawns: Default + Clone {
    fn spawn(&self) -> Result<(), Box<Error>>;
}
