extern crate lazy_static;
extern crate config;

use std::sync::RwLock;
use config::Config;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

pub fn load_config() {
    SETTINGS.write().unwrap().merge(config::File::with_name("settings")).unwrap();
}
