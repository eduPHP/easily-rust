use std::{fs, path::Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Require {
    pub php: String
}

#[derive(Serialize, Deserialize)]
pub struct Composer {
    pub name: String,
    pub require: Require,
}

pub fn exists() -> bool {
    let file_path: String = String::from("composer.json");
    return Path::new(&file_path).exists();
}

pub fn read() -> Composer {
    let file_path: String = String::from("composer.json");

    let content: String = fs::read_to_string(file_path)
        .expect("Unable to read composer.json");

    let composer: Composer = serde_json::from_str(&content)
        .expect("Unable to parse composer.json");

    return composer;
}