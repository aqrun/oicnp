---
title: php autoload之composer篇
date: 2014-06-20 17:42:03
slug: php-autoload-composer

taxonomies:
  categories: ['backend', 'article']
  tags: ['php', 'composer']
---

Composer 是 PHP 依赖管理工具 具体介绍官方网站： https://getcomposer.org/doc/00-intro.md 本篇介绍 小项目如何使用 composer 生成自动加载类文件

## 1. 创建项目 test 目录结构：

```sh
/test/
/vendor/ //第三方包 类库
/module/ // 模块
TestClass.php //模块TestClass类
index.php
composer.json //composer.json
```

//TestClass.php

```json
// composer.json
{
  "name": "ztest", //name 之类其它好多属性 不是必须的
    "autoload": { // autoload 属性用来定义本项目 要自动加载的自定义项
    "psr-4": { //使用psr-4规范 会将内容生成到 vendor/composer/autoload-psr4.php 文件中
      "module\\": "vendor/module/"
    }
  }
}
```

## 2. 运行使用 composer 自动构建 composer 安装方法官网有 进入控制台 路径到所建项目根路径 也就是 composer.json 所在路径 运行： composer install 会自动生成 /vendor/composer/\* //这个目录 5 个文件 会根据 composer.json 配置参数生成相关内容 /vendor/autoload.php 最后项目入口文件

==============

```php
//index.php
addPsr4("module2", "module2的路径");     //这句在项目调试时可以用      临时增加新的命名空间
echo (new TestClass())->name;   //输出 test content
```
