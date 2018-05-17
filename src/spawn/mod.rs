pub use self::spawn::Spawn;

pub mod command;
pub mod spawn;

use std::error::Error;

/// Trait outlining `Spawns`.
///
/// All `Spawn`s must impliment this trait.
/// Similar to `Times`, this cannot be enforced by the compiler, so we must do it by hand.
pub trait Spawns {
    /// Calls the `Spawn` instance.
    fn spawn(&self) -> Result<(), Box<Error>>;
}
