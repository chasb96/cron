pub use self::timer::Timer;

pub mod interval;
pub mod once;
pub mod timer;

use tokio::timer::Error;

/// Trait outlining the needs for a `Times`.
///
/// Any `Times` must impl this trait.
/// However, since this cannot be enforced by the compiler, we will need to enforce this by hand.
pub trait Times {
    /// Create a default `Times` instance.
    /// This instructs how to derive details when details are missing.
    ///
    /// Returns the default of the `Times` instance.
    fn default() -> Self;

    /// Calls the dependent according to the `Times` instance.
    fn call<F: Fn() -> Result<(), Error> + Send + 'static>(&self, f: Box<F>);
}
