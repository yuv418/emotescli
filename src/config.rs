extern crate json;
extern crate keyring;

use directories::ProjectDirs;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub api_key: String,
    pub domain: String
}

impl Config {

    pub fn from(domain: String) -> Option<Config> {
        let service = "emotescli"; // TODO make this a constant

        let keyring = keyring::Keyring::new(service, &domain);
        let api_key = match keyring.get_password() {
            Ok(key) => key,
            Err(er) => return None
        };

        Some(
            Config {
                api_key: api_key,
                domain: domain
            }
        )
    }

    pub fn new(domain: String, api_key: String) -> Option<Config>{
        let service = "emotescli";

        let keyring = keyring::Keyring::new(service, &domain);
        keyring.set_password(&api_key);

        Some(
            Config {
                api_key: api_key,
                domain: domain
            }
        )
    }
}
