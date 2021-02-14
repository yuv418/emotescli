use serde::Serialize;
use serde::de::DeserializeOwned;
use reqwest::blocking::Client;
use reqwest::blocking::multipart::Form;
use reqwest::Method;
use std::collections::HashMap;
use crate::config::Config;
use crate::types::AddMultipartFiles;
use serde_json;
use core::fmt::Debug;


pub fn dispatch<T, U>(config: &Config, verb: &str, method: Method, t: T) -> serde_json::Result<U> where T: Serialize + Debug, U: DeserializeOwned { // Some structs cannot implement AddMultipartFiles. Perhaps this stuff needs to be implemented elsewhere
    let form = make_form(&t, config);
    do_req(config, verb, method, form)
}

fn do_req<U>(config: &Config, verb: &str, method: Method, form: Form) -> serde_json::Result<U> where U: DeserializeOwned {
    let c = Client::new();
    let url = "https://".to_owned() + &config.domain + verb;

    let req = c.request(method, &url)
        .multipart(form)
        .send();

    let res = req.unwrap();

    let text = res.text().unwrap();
    // println!("{:?}", serde_json::from_str::<HashMap<String, serde_json::value::Value>>(&text));

    serde_json::from_str(&text)
}

fn make_form<T>(t: &T, config: &Config) -> Form where T: Serialize + Debug {
    let mut val = serde_json::to_value(t).unwrap();
    let mut form_hash: HashMap<String, serde_json::Value> = serde_json::from_value(val).unwrap_or(HashMap::new()); // Not everything is a string.
    let mut form = Form::new();

    for (key, value) in form_hash {
        let form_insert = match value {
            serde_json::Value::Bool(tf) => { if tf { "1" } else { "0" } }.to_owned(),
            serde_json::Value::Number(num) => num.to_string(),
            serde_json::Value::String ( strin ) => strin,
            _ => String::new(),
        };

        form = form.text(key, form_insert);
    }

    // println!(" form hash {:?}", form);
    form.text("api_key", config.api_key.clone())
}

pub fn dispatch_with_files<T, U>(config: &Config, verb: &str, method: Method, t: T) -> serde_json::Result<U> where T: Serialize + Debug + AddMultipartFiles, U: DeserializeOwned {
    let mut form = make_form(&t, config);
    form = t.add_multipart_files(form);

    do_req(config, verb, method, form)
}

// Create a dispatch file upload method that requires the AddMultiPartFiles and break up the dispatch method
