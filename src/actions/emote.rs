use crate::types::emote::Emote;
use crate::config::Config;

pub fn upload(config: Config, path: &str, name: &str, file_path: &str) {
    let new_emote = match Emote::upload(&config, path.to_string(), name.to_string(), file_path.to_string()) {
        Some(emote) => emote,
        None => panic!("Failed to create emote!")
    };

    println!("Created emote. Access it at https://{}/{}/{}.", &config.domain, path, new_emote.slug);
}

pub fn delete(path: &str) {

}
