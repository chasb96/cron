pub use self::timer::Timer;

pub mod interval;
pub mod once;
pub mod timer;

use std::error::Error;
use tokio::runtime::Runtime;

/// Trait outlining the needs for a `Times`.
///
/// Any `Times` must impl this trait.
/// However, since this cannot be enforced by the compiler, we will need to enforce this by hand.
pub trait Times {
    /// Calls the dependent according to the `Times` instance.
    fn time<F>(&self, runtime: &mut Runtime, closure: F)
    where
        F: Fn() -> Result<(), Box<Error>> + Send + 'static;
}
