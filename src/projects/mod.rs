use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::env;
use serde::{Deserialize, Serialize};
use slug::slugify;

use crate::docker;
use crate::print_help;
use crate::ssl;
use crate::stubs;
use crate::composer;

#[derive(Serialize, Deserialize)]
struct Config {
    aliases: HashMap<String, String>,
}

pub fn try_to_guess() -> String {
    let composer = composer::read();
    let config: Config = config();

    
    if !config.aliases.contains_key(composer.name.trim()) {
        return composer.name.to_string();
    }

    return config.aliases.get(composer.name.trim()).unwrap().to_owned();
}

fn config() -> Config {
    let projects_config_path = format!("{}/.config/easily/projects.json", env!("HOME").to_string());
    let content = fs::read_to_string(projects_config_path).expect("Error reading projects config file");

    let config: Config = serde_json::from_str(&content).expect("Error parsing projects config file");

    return config;
}

#[allow(unused_variables)]

pub fn run(name: &str) {
    docker::create_network_if_it_doesnt_exist("easily");

    // todo: verificar certificados

    // verificar aliases

    if !composer::exists() {
        println!("composer.json not found, please run this command from the project's folder.");
        print_help();
        return;
    }

    let composer = composer::read();
    let project = composer.name;
    let php = &composer.require.php.to_owned()[0..3];
    
    println!("Project {} running PHP {}", project, &php);

    // let home: &str = env!("HOME");
    // let dir: &str = env!("PWD");
    // let name: String = std::env::var("APP_NAME").unwrap();

    // println!("home: {}\nname: {}\nfolder: {}", home, name.trim(), dir);

    create_structure();
}

fn create_structure() {
    let map: HashMap<&str, String> = HashMap::from([
        ("php/8.1/Dockerfile", stubs::php::dockerfile81()),
        ("php/8.2/Dockerfile", stubs::php::dockerfile82()),
        ("nginx/Dockerfile", stubs::nginx::dockerfile()),
        ("nginx/conf.d/default.conf", stubs::nginx::default()),
        ("nginx/includes/https-redirect.conf", stubs::nginx::include_redirect()),
        ("nginx/includes/laravel.conf", stubs::nginx::include_laravel()),
    ]);

    for (path, content) in map {
        create_file(path, content)
    }

    let projects_config_path = format!("{}/.config/easily/projects.json", env!("HOME").to_string());
    if ! Path::new(&projects_config_path).exists() {
        write_string_to_file(
            Path::new(&projects_config_path), 
            r#"{"aliases": {}}"#
        ).expect("Error trying to write projects config file");
    }
}

pub fn create_file(path: &str, content: String) {
    let config_path: String = format!("{}/.config/easily/{}", env!("HOME").to_string(), path);

    create_folder(&config_path);

    let path = Path::new(&config_path);
    write_string_to_file(path, &content).expect("Error trying to write file");
}

pub fn create_folder(full_path: &str) {
    let path = Path::new(&full_path);

    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
}

pub fn write_string_to_file(path: &Path, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(path, data)?;
    Ok(())
}

pub(crate) fn create(name: &str) {
    println!("Creating project {}", name);

    // create certificates

    ssl::certs(slugify(&name).as_str());
    // create compose.yaml
    // run start
}
