extern crate directories;
extern crate keyring;
extern crate json;

use std::env;
// use emotes::types::namespace::Namespace;
use emotes::config::Config;
// use emotes::actions;
use std::io;
use std::io::Read;
use read_input::prelude::*;
use emotes::dispatcher::*;
use emotes::types;
use emotes::types::AddMultipartFiles;
use reqwest::Method;
use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use read_input::prelude::*;
use emotes::actions::config;

#[derive(StructOpt, Debug)]
#[structopt(about = "emotes")]
enum Opt {
    Create {
        #[structopt(subcommand)]
        identifier: CreateResourceType,
    },
    Get {
        #[structopt(subcommand)]
        identifier: IdResourceType,
    },
    Delete {
        #[structopt(subcommand)]
        identifier: IdResourceType,
    }
}

#[derive(Serialize, StructOpt, Debug)]
enum IdResourceType {
    Emote(IdEmote) , User(IdUser), Namespace(IdNamespace)
}

#[derive(Serialize, StructOpt, Debug)] struct IdEmote {path: String, name: String}
#[derive(Serialize, StructOpt, Debug)] struct IdUser {
    name: String,
    #[structopt(skip)]
    delete: String
}
#[derive(Serialize, StructOpt, Debug)] struct IdNamespace {name: String}

#[derive(Serialize, StructOpt, Debug)]
#[structopt(rename_all = "lower")]
enum CreateResourceType {
    Emote(types::Emote), User(types::User), ApiKey(IdUser), Namespace(types::Namespace)
}

fn main() {
    let opt = Opt::from_args();
    // println!("{:#?}", opt);



    let config = match Config::from(None) {
        Some(config) => config,
        None => {
            let mut domain = input().msg("Your domain: ").get();

            let config = match Config::from(Some(domain)) {
                Some(config) => config,
                None => {
                    println!("Sorry, your domain doesn't exist. Let's create it.");
                    match config::create() {
                        Some(config) => config,
                        None => panic!("Couldn't create config. Something is probably wrong with your keychain setup.")
                    }
                }
            };

            config
        }
    };

    let mut response: Option<types::Response> = None;
    match opt {
        Opt::Create { identifier } => {
            match identifier {
                CreateResourceType::User(user) => { // This has issues sometimes.
                    response = Some(dispatch(&config, "/api/users", Method::POST, user).unwrap());
                },
                CreateResourceType::Namespace(namespace)  => {
                    response = Some(dispatch(&config, "/api/namespaces/", Method::POST, namespace).unwrap());
                }, // API Key?
                CreateResourceType::Emote(emote) => {
                    response = Some(dispatch_with_files(&config, "/api/emotes", Method::POST, emote).unwrap());
                }
                CreateResourceType::ApiKey(user) => {
                    response = Some(dispatch(&config, &format!("/api/users/{}/api_key", user.name), Method::POST, types::Blank).unwrap());
                }
                _ => println!("Not implemented"),
            }
        },
        Opt::Get { identifier } => {
            match &identifier {
                IdResourceType::User ( IdUser { name, .. } ) => {
                    let found_resource: types::User = dispatch(&config, "/api/users", Method::GET, name).unwrap();
                    println!("{:#?}", found_resource);
                },
                IdResourceType::Namespace ( IdNamespace { name } ) => {
                    let found_resource: types::Namespace = dispatch(&config, &("/api/namespaces/".to_owned() + name), Method::GET, types::Blank).unwrap();
                    println!("{:#?}", found_resource);
                },
                _ => println!("Getting emotes is not supported at this time. If you want to view emotes, please view a namespace instead."),
            }
        },
        Opt::Delete { mut identifier } => { // TODO add a confirmation prompt for all of these
            match identifier {
                IdResourceType::User(ref mut user) => { // This doesn't work yet ( backend issue! ).
                    let mut confirmation = false;
                    let confirmation_str = input::<String>()
                        .repeat_msg(format!("Are you SURE you want to delete user \"{}\"? There is no going back. (y/N) ", user.name))
                        .default("n".to_owned())
                        .add_test(|yn| yn == "n" || yn == "y")
                        .get();

                    if confirmation_str == "y" {
                        user.delete = user.name.to_string();
                        response = Some(dispatch(&config, "/api/users", Method::DELETE, user).unwrap()); // Undocumented API call... you need to inject a boolean as well
                    }
                    else {
                        println!("Aborting."); // Will exit after this
                    }

                },
                IdResourceType::Namespace ( IdNamespace { ref name } ) => {
                    response = Some(dispatch(&config, &("/api/namespaces/".to_owned() + name), Method::DELETE, types::Blank).unwrap());
                },
                IdResourceType::Emote (ref emote) => {
                    response = Some(dispatch(&config, "/api/emotes", Method::DELETE, emote).unwrap());
                }
                _ => println!("Delete not implemented for this type")
            }
        }
    };

    if let Some(response_unwrap) = response {
        println!("{}", response_unwrap);
    }
    //



}

