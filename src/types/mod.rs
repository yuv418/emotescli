use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;
use reqwest::blocking::multipart::Form;
use std::fmt;

pub trait AddMultipartFiles {
    fn add_multipart_files(&self, form: Form) -> Form {form}
}

#[derive(StructOpt, Debug, Serialize, Deserialize)]
pub struct Namespace {
    #[structopt(skip)]
    #[serde(skip_serializing)]
    #[serde(default)]
    children: Box<Vec<Namespace>>,
    #[structopt(skip)]
    #[serde(skip_serializing)]
    parent: Box<Option<Namespace>>,
    #[structopt(skip)]
    #[serde(skip_serializing)]
    #[serde(default)] // For if we query a user and the API returns an incomplete namespace.
    pub emotes: Vec<Emote>,
    #[structopt(skip)]
    #[serde(skip_serializing)]
    id: u32,
    name: String,
    #[serde(rename(serialize = "path"))]
    slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    id: Option<i32>, // For create namespace, which doesn't use any other value
    msg: Option<String>, // For delete emote, delete namespace
    path: Option<String>, // For create emote
    key: Option<String>, // For create apikey

}

impl fmt::Display for Response {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(id) = &self.id { // Create namespace
            return write!(f, "Created namespace with id {}", id)
        }
        else if let Some(path) = &self.path { // Create emote
            return write!(f, "{} with path {}", self.msg.as_ref().unwrap(), path)
        }
        else if let Some(key) = &self.key { // Create emote
            return write!(f, "API key: {}", key)
        }
        write!(f, "{}", self.msg.as_ref().unwrap())
    }
}

#[derive(StructOpt, Debug, Serialize, Deserialize)]
pub struct Emote {
    #[structopt(skip)]
    pub id: u32,
    #[structopt(skip)]
    pub info: HashMap<String, String>,
    #[serde(skip_deserializing)]
    pub path: String, // Namespace
    pub name: String,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    #[structopt(parse(from_os_str))]
    pub emotes_file: PathBuf, //
    #[serde(skip_deserializing)]
    #[structopt(skip="png")]
    pub r#type: String,
}

impl AddMultipartFiles for Emote {
    fn add_multipart_files(&self, form: Form) -> Form {
        form.file("emotes_file", &self.emotes_file).unwrap()
    }
}

#[derive(StructOpt, Deserialize, Serialize, Debug)]
pub struct User {
    #[structopt(long, short)]
    admin: bool,
    #[structopt(skip)]
    #[serde(skip_serializing)]
    id: u32,
    name: String,
    #[structopt(skip)]
    #[serde(skip_serializing)]
    namespaces: Vec<Namespace>,
    #[structopt(skip)]
    #[serde(skip_deserializing)]
    api_key: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blank;
