use std::{env, os::unix::fs::MetadataExt};

use crate::config::{Config, load};

pub fn compose_global() -> String {
    let config: Config = load();

    let uid = std::fs::metadata("/proc/self").map(|m| m.uid()).unwrap();
    let gid = std::fs::metadata("/proc/self").map(|m| m.gid()).unwrap();
    let server_root = config.path;
    let db_name = "easily";
    format!(
        r##"networks:
    easily:
        external: true

services:
    nginx:
        build:
            context: .
            dockerfile: ../nginx/Dockerfile
            args:
                - UID={uid}
                - GID={gid}
        ports:
            - "80:80"
            - "443:443"
        volumes:
            - ../nginx/conf.d:/etc/nginx/conf.d/
            - ../nginx/includes:/etc/nginx/include
            - ../nginx/certs:/etc/nginx/certs
            - ../nginx/sites:/etc/nginx/sites
            - {server_root}:/var/www/html
        networks:
            - easily

    mysql:
        image: mysql:8
        tty: true
        ports:
            - "3306:3306"
        environment:
            MYSQL_DATABASE: {db_name}
            MYSQL_ROOT_PASSWORD: secret
            SERVICE_TAGS: dev
            SERVICE_NAME: mysql
        entrypoint:
            "sh -c \"
            echo 'SET GLOBAL local_infile=1; CREATE DATABASE IF NOT EXISTS {db_name}; CREATE DATABASE IF NOT EXISTS {db_name}_testing;' > /docker-entrypoint-initdb.d/init.sql;
            /usr/local/bin/docker-entrypoint.sh --character-set-server=utf8mb4 --collation-server=utf8mb4_unicode_ci
            \""
        networks:
            - easily

    redis:
        image: redis:alpine
        ports:
            - "6379:6379"
        networks:
            - easily

    mailhog:
        image: mailhog/mailhog:latest
        ports:
            - "1025:1025"
            - "8025:8025"
        networks:
            - easily
"##
    )
}

pub fn compose(php: &str, _name: &str) -> String {
    let uid = std::fs::metadata("/proc/self").map(|m| m.uid()).unwrap();
    let gid = std::fs::metadata("/proc/self").map(|m| m.gid()).unwrap();
    let server_root = env::current_dir().unwrap();
    let server_root = server_root.display();
    let php_port = "9000";
    format!(
        r##"networks:
    easily:
        external: true

services:
    php:
        build:
            context: .
            dockerfile: ../../php/{php}/Dockerfile
            args:
                - UID={uid}
                - GID={gid}
        ports:
            - {php_port}:{php_port}
        volumes:
            - {server_root}:/var/www/html
        networks:
            - easily

    composer:
        build:
            context: .
            dockerfile: ../../php/{php}/Dockerfile
            args:
                - UID={uid}
                - GID={gid}
        volumes:
            - {server_root}:/var/www/html
        depends_on:
            - php
        entrypoint: [ 'composer', '--ignore-platform-reqs' ]
        networks:
            - easily

    npm:
        image: node:current-alpine
        volumes:
            - {server_root}:/var/www/html
        ports:
            - "3000:3000"
            - "3001:3001"
            - "5173:5173"
        working_dir: /var/www/html
        entrypoint: [ 'npm', 'run', 'watch' ]
        networks:
            - easily

    artisan:
        build:
            context: .
            dockerfile: ../../php/{php}/Dockerfile
            args:
                - UID={uid}
                - GID={gid}
        volumes:
            - {server_root}:/var/www/html
        depends_on:
            - php
        entrypoint: [ 'php', '/var/www/html/artisan' ]
        networks:
            - easily
"##
    )
}
