use std::{
    path::Path,
    process::{Command, Stdio},
};

use slug::slugify;

use crate::{
    config,
    projects::{create_folder, write_string_to_file},
};

fn create_root_ca() {
    let key_path = config::path("ssl/rootCA.key");
    let pem_path = config::path("ssl/rootCA.pem");

    if Path::new(&pem_path).exists() {
        return;
    }

    create_folder(&pem_path);

    Command::new("openssl")
        .args([
            "genrsa",
            "-des3",
            "-passout",
            "pass:secret",
            "-out",
            &key_path,
            "2048",
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    Command::new("openssl")
        .args([
            "req",
            "-x509",
            "-new",
            "-nodes",
            "-key",
            &key_path,
            "-sha256",
            "-days",
            "1825",
            "-passin",
            "pass:secret",
            "-out",
            &pem_path,
            "-subj",
            "/O=Acme/C=CA/ST=Canada/L=Canada/O=IT/CN=server.example.com",
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
}

pub fn certs(name: &str) {
    create_root_ca();
    let domain = project_name_to_domain(&name).to_owned();

    let pem_path = config::path("ssl/rootCA.pem");
    let key_path = config::path("ssl/rootCA.key");
    let keyfile = config::path(format!("nginx/certs/{}.test.key", domain).as_str());
    let csrfile = config::path(format!("nginx/certs/{}.test.csr", domain).as_str());
    let crtfile = config::path(format!("nginx/certs/{}.test.crt", domain).as_str());
    let extfile = config::path(format!("nginx/certs/{}.ext", domain).as_str());

    create_folder(&crtfile);

    if Path::new(&csrfile).exists() {
        return;
    }

    Command::new("openssl")
        .args(["genrsa", "-out", &keyfile, "2048"])
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    Command::new("openssl")
        .args([
            "req",
            "-new",
            "-key",
            &keyfile,
            "-out",
            &csrfile,
            "-subj",
            "/O=Acme/C=CA/ST=Canada/L=Canada/O=IT/CN=server.example.com",
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let ext_content = format!(
        "authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = {}.test",
        domain
    );
    let _ = write_string_to_file(Path::new(&extfile), &ext_content);

    Command::new("openssl")
        .args([
            "x509",
            "-req",
            "-in",
            &csrfile,
            "-CA",
            &pem_path,
            "-CAkey",
            &key_path,
            "-CAcreateserial",
            "-out",
            &crtfile,
            "-days",
            "1825",
            "-sha256",
            "-extfile",
            &extfile,
            "-passin",
            "pass:secret",
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
}

pub fn project_name_to_domain(name: &str) -> String {
    let parts = name.split("/");
    let parts = parts.collect::<Vec<&str>>();

    if parts.len() == 1 {
        return parts.first().unwrap().to_string();
    }

    return format!(
        "{}.{}",
        slugify(parts[1..parts.len()].join("-")),
        parts.first().unwrap()
    );
}
