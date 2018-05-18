pub use self::builder::Builder;

mod builder;
mod worker;

use tokio::runtime::Runtime;

/// Trait to state an object can be built and run
pub trait Builds {
    fn build(self, runtime: &mut Runtime);
}
