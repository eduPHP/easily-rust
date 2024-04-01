use std::{path::Path, process::{Command, Stdio}};

use crate::projects::{create_folder, write_string_to_file};

fn root() {
    let home = env!("HOME");
    let key_path = format!("{}/.config/easily/ssl/rootCA.key", home);
    let pem_path = format!("{}/.config/easily/ssl/rootCA.pem", home);

    if Path::new(&pem_path).exists() {
        return;
    }

    create_folder(&pem_path);

    Command::new("openssl")
        .args(["genrsa", "-des3", "-passout", "pass:secret", "-out", &key_path, "2048"])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    Command::new("openssl")
        .args([
            "req", "-x509", "-new", "-nodes",
            "-key", &key_path,
            "-sha256","-days", "1825", "-passin", "pass:secret",
            "-out", &pem_path,
            "-subj", "/O=Acme/C=CA/ST=Canada/L=Canada/O=IT/CN=server.example.com"
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
}


pub fn certs(domain: &str) {
    root();

    let home = env!("HOME");
    let pem_path = format!("{}/.config/easily/ssl/rootCA.pem", home);
    let key_path = format!("{}/.config/easily/ssl/rootCA.key", home);
    let keyfile = format!("{}/ssl/certs/{}.test.key", home, domain);
    let csrfile = format!("{}/ssl/certs/{}.test.csr", home, domain);
    let crtfile = format!("{}/ssl/certs/{}.test.crt", home, domain);
    let extfile = format!("{}/ssl/certs/{}.ext", home, domain);

    create_folder(&crtfile);

    Command::new("openssl")
        .args(["genrsa", "-out", &keyfile, "2048"])
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    Command::new("openssl")
        .args([
            "req", "-new", 
            "-key", &keyfile,
            "-out", &csrfile,
            "-subj", "/O=Acme/C=CA/ST=Canada/L=Canada/O=IT/CN=server.example.com"
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let ext_content = format!("authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = {}.test", domain);
    let _ = write_string_to_file(Path::new(&extfile), &ext_content);
    
    Command::new("openssl")
    .args([
        "x509", "-req",
        "-in", &csrfile,
        "-CA", &pem_path,
        "-CAkey", &key_path,
        "-CAcreateserial",
        "-out", &crtfile,
        "-days", "1825", "-sha256",
        "-extfile", &extfile,
        "-passin", "pass:secret"
    ])
    .stdout(Stdio::piped())
    .output()
    .unwrap();
}