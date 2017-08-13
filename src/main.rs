extern crate serde;
extern crate serde_json;
extern crate rand;

mod json_file;
mod config;
mod list;
mod randomized_list;

use config::Config;

fn main() {
    // let mut config = Config::new("~/.notifier/config.json".to_string());
    //
    // let mut list = ReminderList::new("~/.notifier/list.json".to_string());
    //
    // config.open().unwrap();
    // list.fill();
}
