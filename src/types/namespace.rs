use super::emote::Emote;
use std::collections::HashMap;
use serde::Deserialize;
use crate::config::Config;
extern crate reqwest;
use reqwest::StatusCode;
use json;
use reqwest::blocking::Client;
use reqwest::blocking::multipart;

#[derive(Debug, Deserialize)]
pub struct Namespace {
    children: Box<Vec<Namespace>>,
    parent: Box<Option<Namespace>>,
    pub emotes: Vec<Emote>,
    id: u32,
    name: String,
    slug: String,
}

impl Namespace {
    pub fn from(config: &Config, path: String) -> Option<Namespace> {
        let mut map = HashMap::new();
        map.insert("api_key", &config.api_key);

        let client = reqwest::blocking::Client::new();
        let body = match client.get(format!("https://{}/api/namespaces/{}", config.domain, path).as_str())
            .form(&map)
            .send() {
                Ok(ref res) if res.status() == StatusCode::NOT_FOUND => {
                    return None
                },
                Ok(res) => res,
                Err(res) => panic!("Res was {:?}", res)
            };

        match body.json() {
            Ok(nmsp) => nmsp,
            Err(er) => panic!("Error: {:?}", er)
        }

    }

    pub fn new(config: &Config, path: String, name: String) -> Option<Namespace> {
        let pathcopy = path.clone();
        let apikey = config.api_key.clone();

        let form = multipart::Form::new()
            .text("api_key", apikey)
            .text("path", path)
            .text("name", name);

        let client = Client::new();
        let body = match client.post(format!("https://{}/api/namespaces", config.domain).as_str())
            .multipart(form)
            .send() {
                Ok(res) => res,
                Err(res) => panic!("Res was {:?}", res)
            };

        match body.text() {
            Err(er) => panic!("Error: {:?}", er),
            Ok(txt) => {
                return Namespace::from(config, pathcopy);
            }
        }

    }
}

fn config() -> Config {
    Config {
        api_key: String::from(""),
        domain: String::from(""),
    }
}
