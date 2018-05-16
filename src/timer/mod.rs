pub use self::timing::Timing;

mod interval;
mod once;
mod timing;

pub trait Timer {
    fn default() -> Self;
}
