use std::collections::HashMap;
use serde::Deserialize;
use crate::config::Config;
use crate::types::namespace::Namespace;
use reqwest::blocking::*;
use mime_guess;
use reqwest::StatusCode;
use json;

#[derive(Debug, Deserialize)]
pub struct Emote {
    pub id: u32,
    pub info: HashMap<String, String>,
    pub name: String,
    pub slug: String,
}

impl Emote {
    pub fn upload(config: &Config, path: String, name: String, file_path: String) -> Option<Emote> {
        let apikey = config.api_key.clone();
        let namecopy = name.clone();
        let namespace = match Namespace::from(config, path.clone()) {
            Some(nmsp) => nmsp,
            None => panic!("The namespace path you provided doesn't exist.")
        };

        let mime = mime_guess::from_path(&file_path);
        let mime = match mime.first() {
            Some(mime) => mime.subtype().to_string(),
            None => panic!("Failed to get mimetype!")
        };

        let form = match multipart::Form::new()
            .text("api_key", apikey)
            .text("path", path)
            .text("name", namecopy)
            .text("type", mime)
            .file("emotes_file", file_path) {
                Ok(form) => form,
                Err(error) => panic!("Error! Your file probably doesn't exist.")
            };

        let client = Client::new();
        let body = match client.post(format!("https://{}/api/emotes", config.domain).as_str())
            .multipart(form)
            .send() {
                Ok(ref res) if res.status() == StatusCode::BAD_REQUEST => return None,
                Ok(res) => res,
                Err(res) => panic!("Res was {:?}", res)
            };



        if let Err(er) = body.text() {
            panic!("Error: {:?}", er);
        }


        for emote in namespace.emotes {
            if emote.name == name {
                return Some(emote)
            }
        }

        None

    }
}
