pub use self::listener::Listener;

pub mod interval;
pub mod listener;
pub mod once;

use std::error::Error;
use tokio::runtime::Runtime;

/// We want Times to impliment Default + Clone for deriving and safety issues.
///   Default allows a useful library serde to find default values for missing information.
///   Clone allows the Times to be used hassle-free asynchronously.
pub trait Waits: Default + Clone {
    fn wait<F>(&self, runtime: &mut Runtime, closure: F)
    where
        F: Fn() -> Result<(), Box<Error>> + Send + 'static;
}
