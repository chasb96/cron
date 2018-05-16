extern crate serde;
extern crate tokio;

#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate serde_json;

mod spawn;
mod timer;

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    Ok(())
}
