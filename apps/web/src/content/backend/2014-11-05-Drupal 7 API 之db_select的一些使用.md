---
title: Drupal 7 API 之db_select的一些使用
date: 2014-11-05 10:30:22
slug: drupal-7-db-select

taxonomies:
  categories: ['backend', 'article']
  tags: ['drupal7', 'db_select', 'php']
---

Drupal 7 数据库 API 增加了 db_select()方法，和 db_query()相比，语法更加清晰，使用也更加方便。目的也不言而喻，就是取代 db_query()。下面我也就列罗出一些经常使用的技巧，可能不够全面，还请大家多多补充。

#### 1. 单表查询（最基本使用方法）

```php
$result = db_select('contact', 'c')  //选择表contact,名一个别名c
 ->fields('c') //查询c表中的字段，后面不跟参数就是查询所有
 ->execute()  //执行操作
 ->fetchAssoc(); //处理结果集

```

#### 2. 条件查询（condition）

```php
$result = db_select('node', 'n')
->fields('n')
->condition('nid', 5,'=')  //nid等于5的条件
->condition('status', 0,'>') // 状态大于0,也就是为真等于1
->condition('uid', array(1,5,7),'IN') //使用IN操作，当然还可以使用 NOT IN
->execute()
->fetchAssoc();
```

#### 3. 联合查询（Join）

```php
$query = db_select('field_data_field_slide','f'); //主表
$query->join('node', 'n', 'fa.entity_id = n.nid'); // 联合node表,条件是nid
$query->condition('f.field_slide_channel_tid',$chanid,'=');
$query->condition('n.status','1','=');  //发布状态
$query->fields('n',array('nid'));  //查询nid
$query->orderBy('n.nid','DESC'); //排序条件
$query->range(0,4); //取得4条数据
$result = $query->execute();
```

#### 4. 添加字段（addField）

```php
$query = db_select('taxonomy_term_hierarchy','h'); //选择表
$query->join('taxonomy_term_data','d','h.tid = d.tid'); // 联合查询
$query->addField('d', 'tid'); // 添加字段
$query->condition('h.parent',0); // 添加条件where 
$query->condition('d.vid',$vid); // 再添加一个条件 and ....
$query->addTag('ditusearch_generate_path'); // 添加Tag 可选项，这个就是方便其他地方可以改变这   个查询$query 比如如果添加了tag 可以使用 hook_query_alter 对其进行查询 如果你使用过views开发，views 也可以了类似的hook
$tid = $query->execute()->fetchCol();

```


#### 5. 分页使用（pager）

```php showLineNumbers
$query = db_select('node', 'n')
    ->condition('type', 'article')
    ->fields('n');
$query = $query->extend('PagerDefault')->limit(2); //limit条件是分页数目
$result = $query->execute();
foreach($result as $res){
    $output .= $res->title;
}
$output .= theme(‘pager’); //添加分页theme
Return $output; 
```


#### 6. 多表联合分页

```php showLineNumbers
$query = db_select('field_data_field_news','fa')->extend('PagerDefault'); //多表联合查询extend条件必须放在db_select之后

$query->join('node','n','fa.entity_id = n.nid');
$query->join('field_data_field_news_date_sort', 'fb', 'fa.entity_id = fb.entity_id');
$query->fields('n', array('nid','title'));
$query->condition('fa.field_news_classify_tid',$tids,'in'); 
$query->condition('n.type','news','='); 
$query->condition('n.status','1','=');     
$query->orderBy('fb.field_news_date_sort_value','DESC');
$query->limit(14);
$result = $query->execute();

```


http://drupalchina.cn/node/2026
