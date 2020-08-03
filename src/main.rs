extern crate directories;
extern crate keyring;
extern crate json;

use std::env;
use emotes_uploader::types::namespace::Namespace;
use emotes_uploader::config::Config;
use emotes_uploader::actions;
use std::io;
use std::io::Read;
use read_input::prelude::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let mut domain = input().msg("Your domain: ").get();

    let config = match Config::from(domain) {
        Some(config) => config,
        None => {
            println!("Sorry, your domain doesn't exist. Let's create it.");
            match actions::config::create() {
                Some(config) => config,
                None => panic!("Couldn't create config. Something is probably wrong with your keychain setup.")
            }
        }
    };

    if args.len() >= 2 {

        let action = &args[1];
        match action.as_str() {
            "emotes" => match args[2].as_str() {
                "upload" => actions::emote::upload(config, &args[3], &args[4], &args[5]),
                "delete" => actions::emote::delete(&args[3]),
                "help" => {
                    println!("\nTry:\n");
                    println!("emotes emotes upload NAMESPACE_PATH EMOTE_NAME EMOTE_PATH");
                    println!("emotes emotes delete (unimplemented)");
                }
                _ => println!("Invalid emote action `{}`", args[2])
            }
            "user" => match args[2].as_str() {
                "create" => actions::user::create(&args[3]),
                "key" => actions::user::key(&args[3]),
                "delete" => actions::user::delete(&args[3]),
                "help" => {
                    println!("\nTry:\n");
                    println!("emotes user create (unimplemented)");
                    println!("emotes user key (unimplemented)");
                    println!("emotes user delete (unimplemented)");
                    println!("emotes user help (unimplemented)");
                }
                _ => println!("Invalid user action `{}`", args[2])
            },
            "namespace" => match args[2].as_str() {
                "create" => actions::namespace::create(config, &args[3], &args[4]),
                "delete" => actions::namespace::delete(args.remove(3)),
                "view" => actions::namespace::view(config, &args[3]),
                "help" => {
                    println!("\nTry:\n");

                    println!("emotes namespace create NAMESPACE_PATH NAME");
                    println!("emotes namespace view NAMESPACE_PATH");

                    println!(
                        "\nThe namespace path (NAMESPACE_PATH) is important. Namespaces always have parent and child namespaces. \
These are separated by slashes. Eg. the namespace test1 can have a child namespace test2, so if you're looking for the namespace test2 then you have to specify the NAMESPACE_PATH as \
test1/test2. This also goes for creating emotes. However, when trying to create a namespace path, you **should not** create a namespace whose parent does not exist. \
In that case, the server will silently create the parent namespace under the name for the child namespace. So if you try creating test1/test2 where test1 doesn't exist and specify the name \
as \"Test 2,\" then you will get the namespace test1 UNDER the name \"Test 2\" AND the program will report that the namespace you tried to create was not created. Just don't do this for now.
That is a server-side bug right now, and it will be fixed later. So you must first create test1 and then create test2. You cannot do both at once."
                    );
                }
                _ => println!("Invalid namespace action `{}`", args[2])
            },
            "help" => {
                println!("\nTry:\n");
                println!("emotes namespace help");
                println!("emotes emotes help");
                println!("emotes user help");

            }
            _ => println!("Invalid action `{}`. Try asking for help with `emotes help`?", action)
        }


    }
    else {
        println!("No action specified. Try asking for help with `emotes help`?");
    }

}

