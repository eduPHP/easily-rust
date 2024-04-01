use std::process::{Command, Stdio};
use std::io;
use std::io::Write;

#[allow(dead_code)]
pub fn ps() {
    let output = Command::new("docker")
        .arg("ps")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");
    
    io::stdout().write_all(&output.stdout).unwrap();
}

pub fn create_network_if_it_doesnt_exist(name: &str) {
    if network_exists(name) {
        return;
    }

    println!("{} network not found, creating it", name);

    Command::new("docker")
        .args(["network", "create", name,"--attachable"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap(); 
}

fn network_exists(name: &str) -> bool {
    let ps_child = Command::new("docker")
        .args(["network", "ls", "--format","\"{{.Name}}\""])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap(); 

    let filtered = Command::new("grep")
        .arg(name)
        .stdin(Stdio::from(ps_child.stdout.unwrap())) // Pipe through.
        .stdout(Stdio::piped())
        .output();

    let response: String = String::from_utf8(filtered.unwrap().stdout).expect("Failed to convert stdout");

    return response.contains(name);
}
