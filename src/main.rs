extern crate serde;
extern crate tokio;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod builder;
mod spawn;
mod timer;

use builder::Builds;
use builder::Config;
use builder::FromFile;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use tokio::prelude::*;
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<Error>> {
    let mut runtime = Runtime::new()?;

    let config = Config::from_file(default_path().unwrap())?;

    config.build(&mut runtime)?;

    runtime.shutdown_on_idle().wait().unwrap();

    Ok(())
}

fn default_path() -> Option<PathBuf> {
    let mut path = env::home_dir()?;
    path.push("tasker.json");

    Some(path)
}
