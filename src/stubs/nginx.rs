use crate::ssl::project_name_to_domain;

pub fn default() -> String {
    r"include include/https-redirect.conf;
map $ssl_server_name $ssl_domain_name {
    volatile;
    hostnames;
    default 'localhost.test';
    ~^(?<domain>[^.]+)\.test$ $domain.test;
}

include sites/*.conf;
"
    .to_owned()
}
pub fn dockerfile() -> String {
    r#"FROM nginx:stable-alpine

ARG UID
ARG GID

ENV UID=${UID}
ENV GID=${GID}

# MacOS staff group's gid is 20, so is the dialout group in alpine linux. We're not using it, let's just remove it.
RUN delgroup dialout

RUN addgroup -g ${GID} --system laravel
RUN adduser -G laravel --system -D -s /bin/sh -u ${UID} laravel
RUN sed -i "s/user  nginx/user laravel/g" /etc/nginx/nginx.conf

RUN mkdir -p /var/www/html"#.to_owned()
}
pub fn include_redirect() -> String {
    return r"server {
    listen 80 default_server;
    server_name _;
    return 301 https://$host$request_uri;
}"
    .to_owned();
}
pub fn project(name: &str) -> String {
    let domain = project_name_to_domain(name);
    format!(
        r#"server {{
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name "{domain}.test";
    index index.php index.html;
    root /var/www/html/fidelis/public;

    location / {{
        try_files $uri $uri/ /index.php?$query_string;
    }}

    ssl_certificate /etc/nginx/certs/{name}.crt;
    ssl_certificate_key /etc/nginx/certs/{name}.key;

    location ~ \\.php$ {{
        try_files $uri =404;
        fastcgi_split_path_info ^(.+\.php)(/.+)$;
        fastcgi_pass {name}-php:9000;
        fastcgi_index index.php;
        include fastcgi_params;
        fastcgi_param SCRIPT_FILENAME $document_root$fastcgi_script_name;
        fastcgi_param PATH_INFO $fastcgi_path_info;
    }}
}}
"#
    )
}
