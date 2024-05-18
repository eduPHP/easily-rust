use crate::{cmd::{run, grep}, config};

#[allow(dead_code)]
pub fn ps() {
    run_command(&["ps"]);
}

pub fn create_network_if_it_doesnt_exist(name: &str) {
    if network_exists(name) {
        return;
    }

    println!("{} network not found, creating it", name);

    run("docker", &["network", "create", name,"--attachable"]).unwrap(); 
}

fn network_exists(name: &str) -> bool {
    let ps_child = run(
        "docker", 
        &["network", "ls", "--format","\"{{.Name}}\""]
    ).unwrap();

    return grep(ps_child.stdout, name);
}


pub fn start(name: &str, yaml: &str) {
    let temp_name = format!("{}-test", name);
    let yaml = &config::path(yaml);
    let global_yaml = &config::path("projects/global-compose.yaml");
    
    let args = ["compose", "-f", global_yaml, "-p", "easily", "up", "-d"];
    run_command(&args);
    
    let args = ["compose", "-f", yaml, "-p", &temp_name, "up", "-d"];
    run_command(&args);
}

pub fn stop(name: &str, yaml: &str) {
    let temp_name = format!("{}-test", name);
    let yaml = &config::path(yaml);
    let args = ["compose", "-f", yaml, "-p", &temp_name, "kill"];
    run_command(&args);
}

pub fn halt() {
    let global_yaml = &config::path("projects/global-compose.yaml");
    let args = ["compose", "-f", global_yaml, "-p", "easily", "kill"];
    run_command(&args);
}

fn run_command(args: &[&str]) {
    let mut output = run("docker", args).unwrap();

    let ecode = output.wait()
        .expect("failed to wait on child");

    assert!(ecode.success());
}