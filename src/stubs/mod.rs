pub mod nginx {
    pub fn dockerfile() -> String {
        return r#"FROM nginx:stable-alpine

ARG UID
ARG GID

ENV UID=${UID}
ENV GID=${GID}

# MacOS staff group's gid is 20, so is the dialout group in alpine linux. We're not using it, let's just remove it.
RUN delgroup dialout

RUN addgroup -g ${GID} --system laravel
RUN adduser -G laravel --system -D -s /bin/sh -u ${UID} laravel
RUN sed -i "s/user  nginx/user laravel/g" /etc/nginx/nginx.conf

RUN mkdir -p /var/www/html"#.to_owned();
    }
    pub fn default () -> String {
        return r"include include/https-redirect.conf;
map $ssl_server_name $ssl_domain_name {
    volatile;
    hostnames;
    default 'localhost.test';
    ~^(?<domain>[^.]+)\.test$ $domain.test;
}
server {
    include include/laravel.conf;
}
".to_owned();
    }

    pub fn include_redirect() -> String {
        return r"server {
    listen 80 default_server;
    server_name _;
    return 301 https://$host$request_uri;
}".to_owned();
    }

    pub fn include_laravel() -> String {
        return r#"listen 443 ssl http2;
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
}"#.to_owned();
    }
}

pub mod php {
    pub fn dockerfile81() -> String {
        return r#"FROM php:8.1-fpm-alpine

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
"#.to_owned();
    }

    pub fn dockerfile82() -> String {
        return r#"FROM php:8.2-fpm-alpine

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
"#.to_owned();
    }
}