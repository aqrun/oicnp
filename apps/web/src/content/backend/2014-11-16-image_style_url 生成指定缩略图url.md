---
title: image_style_url 生成指定缩略图url
description: 'modules/image/image.module'
slug: image-style-url-thumb

taxonomies:
  categories: ['backend', 'article']
  tags: ['php', 'drupal7', 'image_style_url']
---

modules/image/image.module

```php
function image_style_url($style_name, $path) {...}

//example:
$imgItem = field_get_items('node', $node, 'field_image');
$imgUrl = image_style_url("com_introduce", $imgItem[0]['uri']);

```
