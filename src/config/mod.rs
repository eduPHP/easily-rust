use std::{collections::HashMap, env, path::Path};

use serde::{Deserialize, Serialize};

pub fn path(path: &str) -> String {
    return format!("{}/.config/easily/{}", env!("HOME").to_string(), path).replace("//", "/");
}

pub fn parent_dir() -> String {
    let current = current_dir();
    return Path::new(&current)
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
}

pub fn current_dir() -> String {
    let server_root = env::current_dir().unwrap();
    return server_root.display().to_string();
}

pub fn load() -> Config {
    let conf: Config = confy::load("easily", "config").unwrap();

    return conf;
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path: String,
    pub aliases: HashMap<String, String>,
    pub php_services: HashMap<String, i32>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: parent_dir(),
            aliases: HashMap::new(),
            php_services: HashMap::new(),
        }
    }
}

pub trait Save {
    fn save(&self);
}

impl Save for Config {
    fn save(&self) {
        confy::store("easily", "config", &self).unwrap();
    }
}
