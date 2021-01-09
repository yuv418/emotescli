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

    pub fn from(domain: Option<String>) -> Option<Config> {
        let service = "emotescli"; // TODO make this a constant

        if let Some(unwrap_domain) = domain {
            return Self::from_domain(unwrap_domain);
        }
        else {
            let default_domain = keyring::Keyring::new(service, "default");
            let unwrap_domain = match default_domain.get_password() {
                Ok(domain) => domain,
                Err(er) => return None
            };

            return Self::from_domain(unwrap_domain);
        }

        None

    }

    fn from_domain(domain: String) -> Option<Config> {
        let service = "emotescli"; // TODO make this a constant

        let domain_key = keyring::Keyring::new(service, &domain);
        let api_key = match domain_key.get_password() {
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

        let default_keyring = keyring::Keyring::new(service, "default");
        default_keyring.set_password(&domain);


        Some(
            Config {
                api_key: api_key,
                domain: domain
            }
        )
    }
}
