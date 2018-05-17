pub mod worker;

use tokio::runtime::Runtime;

pub trait Builds {
    fn build(self, runtime: &mut Runtime);
}
