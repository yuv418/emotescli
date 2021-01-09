use crate::types::emote::Emote;
use crate::config::Config;

pub fn upload(config: Config, path: &str, name: &str, file_path: &str) {
    let new_emote = match Emote::upload(&config, path.to_string(), name.to_string(), file_path.to_string()) {
        Some(emote) => emote,
        None => panic!("Failed to create emote! It is possible your emote already exists.")
    };

    println!("Created emote. Access it at https://{}/{}/{}.", &config.domain, path, new_emote.slug);
}

pub fn delete(config: Config, path: &str, name: &str) {

    let deleted_response = match Emote::delete(&config, path.to_string(), name.to_string()) {
        Ok(ok) => println!("Deleted emote."),
        Err(err) => panic!("Failed to delete emote! It is possible the emote doesn't exist/was already deleted.")
    };

}
