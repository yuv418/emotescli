use crate::config::Config;
use read_input::prelude::*;

pub fn configure() {
    create();
}

pub fn create() -> Option<Config> {
    let domain: String = input().msg("Emotes domain: ").get();
    let api_key: String = input().msg(format!("API key for domain {}: ", domain)).get();

    Config::new(domain, api_key)
}
