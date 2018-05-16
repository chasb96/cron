pub use self::spawn::Spawn;

mod command;
mod spawn;

use std::error::Error;

/// Trait outlining `Spawns`.
///
/// All `Spawn`s must impliment this trait.
/// Similar to `Timer`, this cannot be enforced by the compiler, so we must do it by hand.
pub trait Spawns {
    /// Create a default `Spawns` instance, giving instructions on how to build a spawn with missing details.
    ///
    /// Returns the default `Spawn` instance.
    fn default() -> Self;

    /// Calls the `Spawn` instance.
    fn call(&self) -> Result<(), Box<Error>>;
}
