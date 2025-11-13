---
title: Drupal 7 在 nginx服务器 子目录及虚拟机访问时配置伪静态
date: 2016-06-22 15:33:10
slug: drupal-7-nginx-sub-dir

taxonomies:
  categories: ['backend', 'article']
  tags: ['nginx', 'drupal7']
---

## 文件目录结构：

    /nginx
    /nginx/conf
    /nginx/conf/nginx.conf         #默认nginx配置文件
    /nginx/conf/vhost
    /nginx/conf/vhost/localhost.conf    #
    /nginx/conf/vhost/*.conf       #各虚拟主机配置文件

以下列出三个主要配置文件内容：

```ini showLineNumbers
#＝＝＝＝＝＝＝＝＝＝＝＝＝
#默认配置文件
#nginx.conf
#user  nobody;
worker_processes  1;
#error_log  logs/error.log;
#error_log  logs/error.log  notice;
#error_log  logs/error.log  info;
#pid        logs/nginx.pid;
events {
    worker_connections  1024;
}
http {
    include       mime.types;
    default_type  application/octet-stream;
    autoindex        on;
    sendfile        on;
    keepalive_timeout  65;
    include tools-*.conf;  #引入各种配置文件
    include vhost/*.conf; 
}
#==============================

#========================
#默认访问控制
#localhost.conf
server {
    listen 80; # IPv4
    server_name localhost;
    ## Parameterization using hostname of access and log filenames.
    access_log logs/localhost_access.log;
    error_log logs/localhost_error.log;
    ## Root and index files.
    root D:/xampp/htdocs;
    index  index.php index.html index.htm;
    ## If no favicon exists return a 204 (no content error).
    location = /favicon.ico {
        try_files $uri =204;
        log_not_found off;
        access_log off;
    }
    ## Don't log robots.txt requests.
    location = /robots.txt {
        allow all;
        log_not_found off;
        access_log off;
    }
    ## Try the requested URI as files before handling it to PHP.
        location / {
    
        } # / location

        ## Regular PHP processing.
        location ~ \.php$ {
            try_files  $uri =404;
            fastcgi_pass   127.0.0.1:9000;
            fastcgi_index  index.php;
            fastcgi_param  SCRIPT_FILENAME    $document_root$fastcgi_script_name;
            include        fastcgi_params;
        }

        ## Static files
        location ~* \.(?:css|gif|htc|ico|js|jpe?g|png|swf)$ {
            expires max;
            log_not_found off;
            ## No need to bleed constant updates. Send the all shebang in one
            ## fell swoop.
            tcp_nodelay off;
            ## Set the OS file cache.
            open_file_cache max=1000 inactive=120s;
            open_file_cache_valid 45s;
            open_file_cache_min_uses 2;
            open_file_cache_errors off;
        }

        ## Keep a tab on the 'big' static files.
        location ~* ^.+\.(?:ogg|pdf|pptx?)$ {
            expires 30d;
            ## No need to bleed constant updates. Send the all shebang in one
            ## fell swoop.
            tcp_nodelay off;
        }
        #子目录访问 rewrite方式   
        #   http://localhost/drupal/user         会rewrite到       http://localhost/drupal/index.php?q=user
        location /drupal{
                #使用“-f”和“!-f”运算符检查文件是否存在
                #使用“-d”和“!-d”运算符检查目录是否存在
            if (!-f $request_filename) {
                rewrite  ^/drupal/(.*)$  /drupal/index.php?q=$1  last;
                break;
            }
            if (!-d $request_filename) {
                rewrite  ^/drupal/(.*)$  /drupal/index.php?q=$1  last;
                break;
            }
        }
} # end http server
#==================================


#===================================
#  www.drupal.my    虚拟跟径指向  localhost/drupal
#www.drupal.my.conf
server {
    listen   80; ## listen for ipv4; this line is default and implied

    root D:/xampp/htdocs/drupal;
    index index.html index.php;

    server_name www.drupal.my;
    location / {
            index  index.php index.html;
            #rewrite
            # 使用“-e”和“!-e”运算符检查文件、目录或符号链接是否存在；
            if (!-e $request_filename) {
                rewrite ^/(.*)$ /index.php?q=$1 last;
            }
        }
    
    error_page 404 /404.php;

    location ~ \.php$ {
        try_files  $uri =404;
        fastcgi_pass   127.0.0.1:9000;
        fastcgi_index  index.php;
        fastcgi_param  SCRIPT_FILENAME    $document_root$fastcgi_script_name;
        include        fastcgi_params;
    }
    #rewrite rules
    #include vhost/www.drupal.my.rules;


}
```

