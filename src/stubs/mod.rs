pub mod nginx {
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
    pub fn default () -> String {
        r"include include/https-redirect.conf;
map $ssl_server_name $ssl_domain_name {
    volatile;
    hostnames;
    default 'localhost.test';
    ~^(?<domain>[^.]+)\.test$ $domain.test;
}
server {
    include include/laravel.conf;
}
".to_owned()
    }

    pub fn include_redirect() -> String {
        return r"server {
    listen 80 default_server;
    server_name _;
    return 301 https://$host$request_uri;
}".to_owned();
    }

    pub fn include_laravel() -> String {
        r#"listen 443 ssl http2;
listen [::]:443 ssl http2;
server_name "~^(?<app>.+)\.test";
index index.php index.html;
root /var/www/html/public;

location / {
    try_files $uri $uri/ /index.php?$query_string;
}

ssl_certificate /etc/nginx/certs/$ssl_domain_name.crt;
ssl_certificate_key /etc/nginx/certs/$ssl_domain_name.key;

location ~ \.php$ {
    try_files $uri =404;
    fastcgi_split_path_info ^(.+\.php)(/.+)$;
    fastcgi_pass php:9000;
    fastcgi_index index.php;
    include fastcgi_params;
    fastcgi_param SCRIPT_FILENAME $document_root$fastcgi_script_name;
    fastcgi_param PATH_INFO $fastcgi_path_info;
}"#.to_owned()
    }
}

pub mod php {
    pub fn dockerfile81() -> String {
        r#"FROM php:8.1-fpm-alpine

ARG UID
ARG GID

ENV UID=${UID}
ENV GID=${GID}

RUN mkdir -p /var/www/html

WORKDIR /var/www/html

COPY --from=composer:latest /usr/bin/composer /usr/local/bin/composer

# MacOS staff group's gid is 20, so is the dialout group in alpine linux. We're not using it, let's just remove it.
RUN delgroup dialout

RUN addgroup -g ${GID} --system laravel
RUN adduser -G laravel --system -D -s /bin/sh -u ${UID} laravel

RUN sed -i "s/user = www-data/user = laravel/g" /usr/local/etc/php-fpm.d/www.conf
RUN sed -i "s/group = www-data/group = laravel/g" /usr/local/etc/php-fpm.d/www.conf
RUN echo "php_admin_flag[log_errors] = on" >> /usr/local/etc/php-fpm.d/www.conf

RUN apk add --no-cache libpng-dev libwebp-dev libjpeg-turbo-dev freetype-dev libzip-dev git && \
            docker-php-ext-configure gd --enable-gd --with-freetype --with-jpeg --with-webp && \
            docker-php-ext-install gd && \
            docker-php-ext-configure pcntl --enable-pcntl && \
            docker-php-ext-install pcntl && \
            docker-php-ext-install pdo pdo_mysql zip

RUN mkdir -p /usr/src/php/ext/redis \
    && curl -L https://github.com/phpredis/phpredis/archive/5.3.4.tar.gz | tar xvz -C /usr/src/php/ext/redis --strip 1 \
    && echo 'redis' >> /usr/src/php-available-exts \
    && docker-php-ext-install redis

USER laravel

CMD ["php-fpm", "-y", "/usr/local/etc/php-fpm.conf", "-R"]
"#.to_owned()
    }

    pub fn dockerfile82() -> String {
        r#"FROM php:8.2-fpm-alpine

ARG UID
ARG GID

ENV UID=${UID}
ENV GID=${GID}

RUN mkdir -p /var/www/html

WORKDIR /var/www/html

COPY --from=composer:latest /usr/bin/composer /usr/local/bin/composer

# MacOS staff group's gid is 20, so is the dialout group in alpine linux. We're not using it, let's just remove it.
RUN delgroup dialout

RUN addgroup -g ${GID} --system laravel
RUN adduser -G laravel --system -D -s /bin/sh -u ${UID} laravel

RUN sed -i "s/user = www-data/user = laravel/g" /usr/local/etc/php-fpm.d/www.conf
RUN sed -i "s/group = www-data/group = laravel/g" /usr/local/etc/php-fpm.d/www.conf
RUN echo "php_admin_flag[log_errors] = on" >> /usr/local/etc/php-fpm.d/www.conf

RUN apk add --no-cache libpng-dev libwebp-dev libjpeg-turbo-dev freetype-dev libzip-dev git && \
            docker-php-ext-configure gd --enable-gd --with-freetype --with-jpeg --with-webp && \
            docker-php-ext-install gd && \
            docker-php-ext-configure pcntl --enable-pcntl && \
            docker-php-ext-install pcntl && \
            docker-php-ext-install pdo pdo_mysql zip

RUN mkdir -p /usr/src/php/ext/redis \
    && curl -L https://github.com/phpredis/phpredis/archive/5.3.4.tar.gz | tar xvz -C /usr/src/php/ext/redis --strip 1 \
    && echo 'redis' >> /usr/src/php-available-exts \
    && docker-php-ext-install redis
    
USER laravel

CMD ["php-fpm", "-y", "/usr/local/etc/php-fpm.conf", "-R"]
"#.to_owned()
    }
}

pub mod docker {
    use std::{env, os::unix::fs::MetadataExt};

    use crate::config::{load, Config};
    
    pub fn compose_global() -> String {
        let config: Config = load();

        let uid = std::fs::metadata("/proc/self").map(|m| m.uid()).unwrap();
        let gid = std::fs::metadata("/proc/self").map(|m| m.gid()).unwrap();
        let server_root = config.path;
        let db_name = "easily";
        format!(r##"networks:
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
"##)
    }

    pub fn compose(php: &str, _name: &str) -> String {
        let uid = std::fs::metadata("/proc/self").map(|m| m.uid()).unwrap();
        let gid = std::fs::metadata("/proc/self").map(|m| m.gid()).unwrap();
        let server_root = env::current_dir().unwrap();
        let server_root = server_root.display();
        format!(r##"networks:
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
            - "9000:9000"
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
"##)
    }
}