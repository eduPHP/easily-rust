use std::{
    io,
    process::{Child, ChildStdout, Command, Stdio},
};

pub fn run(command: &str, args: &[&str]) -> io::Result<Child> {
    return Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn();
}

pub fn grep(output: Option<ChildStdout>, search: &str) -> bool {
    let filtered = Command::new("grep")
        .arg(search)
        .stdin(Stdio::from(output.unwrap())) // Pipe through.
        .stdout(Stdio::piped())
        .output();

    let response: String =
        String::from_utf8(filtered.unwrap().stdout).expect("Failed to convert stdout");

    return response.contains(search);
}
