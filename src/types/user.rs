extern crate reqwest;

use super::namespace::Namespace;
use std::collections::HashMap;
use serde::Deserialize;
use crate::config::Config;

#[derive(Debug)]
pub struct User {
    admin: bool,
    id: u32,
    name: String,
    namespaces: Vec<Namespace>,
    api_key: String
}


impl User {
    pub fn from(config: &Config, id: u32) -> Option<User> {
        println!("Here!");
        let mut map = HashMap::new();
        let id_s = id.to_string();
        map.insert("api_key", &config.api_key);
        map.insert("id", &id_s);

        let client = reqwest::blocking::Client::new();
        let body = match client.get(format!("https://{}/api/users", &config.domain).as_str())
            .form(&map)
            .send() {
                Ok(res) => res,
                Err(res) => panic!("Res was {:?}", res)
            };


        match body.text() {
            Ok(text) => {

                let info = json::parse(&text).unwrap();
                let user = User {
                    admin: match info["admin"].as_bool() {
                        Some(res) => res,
                        None => panic!("User admin attr not found."),
                    },
                    id,
                    api_key: config.api_key.clone(),
                    name: match info["name"].as_str() {
                        Some(res) => String::from(res),
                        None => panic!("User name not found."),
                    },
                    namespaces: vec!(),
                };

                Some(user)
            },
            Err(d) => None
        }
    }
}
