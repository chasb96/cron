use std::sync::Mutex;
use tokio::runtime::Runtime;

lazy_static! {
    pub static ref RUNTIME: Mutex<Runtime> = Mutex::new(Runtime::new().unwrap());
}
