extern crate config;
extern crate lazy_static;

use config::Config;
use std::sync::RwLock;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

pub fn load_config() {
    SETTINGS
        .write()
        .unwrap()
        .merge(config::File::with_name("settings"))
        .unwrap();
}
