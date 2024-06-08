use std::collections::HashMap;
use std::io;
use std::path::Path;

use regex::Regex;

use crate::composer;
use crate::config;
use crate::config::Config;
use crate::config::load;
use crate::config::parent_dir;
use crate::config::path;
use crate::config::Save;
use crate::docker;
use crate::msg;
use crate::ssl;
use crate::stubs;

pub fn try_to_guess() -> String {
    if !composer::exists() {
        msg::panic("Could not guess the project name, please input it as an argument.");
    }

    let composer = composer::read();

    let mut config: Config = load();
    config
        .aliases
        .insert("s90dev/fidelis-elite".to_owned(), "fidelis".to_owned());
    config.save();

    if !config.aliases.contains_key(composer.name.trim()) {
        return composer.name.to_string();
    }

    return config.aliases.get(composer.name.trim()).unwrap().to_owned();
}

// fn config() -> Config {
//     let projects_config_path = format!("{}/.config/easily/projects.json", env!("HOME").to_string());
//     let content = fs::read_to_string(projects_config_path).expect("Error reading projects config file");

//     let config: Config = serde_json::from_str(&content).expect("Error parsing projects config file");

//     return config;
// }

pub fn start(name: &str) {
    msg::info(format!("Starting project {}", name).as_str());
    docker::create_network_if_it_doesnt_exist("easily");
    create_structure();
    ssl::certs(&name);
    let yaml = format!("projects/{}/compose.yaml", name);
    let _ = docker::start(name, &yaml);
    println!();
}

fn create_structure() {
    let map: HashMap<&str, String> = HashMap::from([
        ("php/8.1/Dockerfile", stubs::php::dockerfile81()),
        ("php/8.2/Dockerfile", stubs::php::dockerfile82()),
        ("nginx/Dockerfile", stubs::nginx::dockerfile()),
        ("nginx/conf.d/default.conf", stubs::nginx::default()),
        (
            "nginx/includes/https-redirect.conf",
            stubs::nginx::include_redirect(),
        ),
        // ("nginx/includes/laravel.conf", stubs::nginx::include_laravel()),
        (
            "projects/global-compose.yaml",
            stubs::docker::compose_global(),
        ),
    ]);

    for (path, content) in map {
        create_file(path, content)
    }

    let projects_config_path = config::path("projects.json");
    if !Path::new(&projects_config_path).exists() {
        write_string_to_file(Path::new(&projects_config_path), r#"{"aliases": {}}"#)
            .expect("Error trying to write projects config file");
    }
}

pub fn create_file(path: &str, content: String) {
    let config_path: String = config::path(path);

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

pub fn create(name: &str) {
    msg::info(format!("Creating project {}", name).as_str());
    ssl::certs(&name);
    let composer = composer::read();
    let php = get_php_version_from_composer(&composer.require.php);

    // replace php version option to specific
    // create compose.yaml
    let yaml = format!("projects/{}/compose.yaml", name);
    if !Path::new(&path(&yaml)).exists() {
        create_file(&yaml, stubs::docker::compose(&php, &name));
    }
    // create nginx file
    let server = format!("nginx/sites/{}.conf", name);
    // if !Path::new(&path(&server)).exists() {
    create_file(&server, stubs::nginx::project(name));
    // }

    let _ = docker::start(name, &yaml);
    println!();
}

pub fn stop(name: &str) {
    msg::info(format!("Stopping project {}", name).as_str());
    let yaml = format!("projects/{}/compose.yaml", name);
    let _ = docker::stop(name, &yaml);
    println!();
}

pub fn halt(name: &str) {
    docker::halt();
    stop(name);
    println!();
}

fn get_php_version_from_composer(version_str: &str) -> String {
    let versions = version_str.split("|").collect::<Vec<&str>>();
    let regex = Regex::new(r#"[^\d.]"#).unwrap();
    return if versions.len() > 1 {
        let version = versions.last().unwrap().to_owned();
        regex.replace_all(version, "").trim()[0..3].to_string()
    } else {
        regex.replace_all(&version_str, "")[0..3].to_string()
    };
}

pub fn init() {
    create_structure();
    // perguntar pasta (padrao, um nivel acima da atual)
    let parent = parent_dir();

    println!("Plase, input the folder where your projects are stored: ({parent})");
    let mut buffer = String::from(parent);
    io::stdin().read_line(&mut buffer).unwrap();
    let input_folder = buffer.trim();
    if !Path::new(&input_folder).exists() {
        println!("Path \"{input_folder}\" doesn't exist!");
        init();
        return;
    }

    // let mut conf = load();
    // conf.path = input_folder.to_string();
    // set("path", input_folder.to_owned());
    // criar web, mysql, redis, mailhog
}
