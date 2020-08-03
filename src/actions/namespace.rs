use crate::config::Config;
use crate::types::namespace::Namespace;

pub fn create(config: Config, path: &str, name: &str) {
    let new_namespace = Namespace::new(&config, path.to_string(), name.to_string());

    match new_namespace {
        Some(namespace) => view(config, path),
        None => println!("The namespace could not be created. It means that your namespace's parent (or parents of that) don't exist or the namespace already exists.")
    }

}

pub fn view(config: Config, path: &str) {
    let view_namespace = Namespace::from(&config, path.to_string());
    match view_namespace {
        Some(namespace) => println!("The namespace you requested was: {:#?}", namespace),
        None => println!("The namespace you requested was not found.")
    }
}

pub fn delete(path: String) {

}
