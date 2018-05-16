pub use self::timing::Timing;

mod interval;
mod once;
mod timing;

use tokio::timer::Error;

/// Trait outlining the needs for a `Timer`.
///
/// Any `Timer` must impl this trait.
/// However, since this cannot be enforced by the compiler, we will need to enforce this by hand.
pub trait Timer {
    /// Create a default `Timer` instance.
    /// This instructs how to derive details when details are missing.
    ///
    /// Returns the default of the `Timer` instance.
    fn default() -> Self;

    /// Calls the dependent according to the `Timer` instance.
    fn call<F: Fn() -> Result<(), Error> + Send + 'static>(&self, f: Box<F>);
}
